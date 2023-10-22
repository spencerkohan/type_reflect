use crate::attribute_utils::*;
use crate::type_def::InflectionTokenProvider;
// use crate::utils::*;
use type_reflect_core::EnumType;
use type_reflect_core::Inflection;

use super::{syn_type_utils::*, type_utils::EnumCaseBridge, RustTypeEmitter};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    // parse::{Parse, ParseStream},
    Attribute,
    ItemEnum,
    Result,
};
use type_reflect_core::*;

#[derive(Clone, Debug)]
pub struct EnumDef {
    pub tokens: TokenStream,
    pub ident: Ident,
    pub enum_type: EnumType,
    pub inflection: Inflection,
    pub cases: Vec<EnumCase>,
}

fn extract_cases(item: &ItemEnum) -> Result<Vec<EnumCase>> {
    (&item.variants)
        .into_iter()
        .map(|case| {
            let name = format!("{}", case.ident);
            let inflection: Inflection = match RenameAllAttr::from_attrs(&case.attrs) {
                Err(e) => {
                    eprintln!(
                        "Error extracting inflection: {} from attributes: {:#?}",
                        e, &case.attrs
                    );
                    Inflection::None
                }
                Ok(rename_all) => {
                    // println!(
                    //     "Extracted inflection: {:?} from attributes: {:#?}",
                    //     rename_all, &case.attrs
                    // );
                    rename_all.rename_all
                }
            };
            match &case.fields {
                syn::Fields::Named(fileds) => Ok(EnumCase {
                    name,
                    type_: EnumCaseType::Struct(fileds.to_struct_members()?),
                    inflection,
                }),
                syn::Fields::Unnamed(fields) => Ok(EnumCase {
                    name,
                    type_: EnumCaseType::Tuple(fields.to_tuple_members()?),
                    inflection,
                }),
                syn::Fields::Unit => Ok(EnumCase {
                    name,
                    type_: EnumCaseType::Simple,
                    inflection,
                }),
            }
        })
        .collect()
}

impl EnumDef {
    pub fn new(item: &ItemEnum) -> Result<Self> {
        let attributes = EnumAttr::from_attrs(&item.attrs)?;
        let rename_attr = RenameAllAttr::from_attrs(&item.attrs)?;

        let cases = extract_cases(&item)?;

        let enum_type = match (&cases).into_iter().fold(false, |input, case| {
            input
                || if let EnumCaseType::Simple = case.type_ {
                    false
                } else {
                    true
                }
        }) {
            // false indicates it is not complex
            false => EnumType::Simple,
            // true indicates the type is complex
            true => {
                let case_key = match attributes.tag {
                    Some(key) => key,
                    None => {
                        return Err(syn::Error::new(
                            item.ident.span().clone(),
                            r#"The Serde "tag" attribute is required for Enum declarations using the Reflect derive macro.  I.e. #[serde(tag="case")]"#,
                        ))
                    }
                };

                let content_key = attributes.content;
                EnumType::Complex {
                    case_key,
                    content_key,
                }
            }
        };

        Ok(Self {
            tokens: quote! { #item },
            ident: item.ident.clone(),
            enum_type,
            inflection: rename_attr.rename_all,
            cases,
        })
    }

    pub fn emit_cases(&self) -> TokenStream {
        let cases: Vec<TokenStream> = (&self.cases)
            .into_iter()
            .map(|case| {
                let name = &case.name;
                let type_ = case.type_.emit_case();
                let rename_all = &case.inflection.to_tokens();
                quote! {
                    EnumCase {
                        name: #name.to_string(),
                        type_: #type_,
                        inflection: #rename_all,
                    }
                }
            })
            .collect();
        quote! {
            #(#cases),*
        }
    }

    pub fn emit(&self) -> TokenStream {
        let ident = &self.ident();
        let name_literal = format!("{}", ident);
        let cases = &self.emit_cases();
        let rust = format!("{}", self.tokens());

        let enum_type = match &self.enum_type {
            EnumType::Simple => quote! {EnumType::Simple},
            EnumType::Complex {
                case_key,
                content_key,
            } => match content_key {
                Some(content_key) => quote! {
                    EnumType::Complex { case_key: #case_key.to_string(), content_key: Some(#content_key.to_string()) }
                },
                None => quote! {
                    EnumType::Complex { case_key: #case_key.to_string(), content_key: None }
                },
            },
        };

        let inflection = &self.inflection.to_tokens();

        quote! {

            impl Emittable for #ident {
                fn emit_with<E: TypeEmitter>() -> String {
                    E::emit_enum::<Self>()
                }
            }

            impl EnumReflectionType for #ident {
                fn name() -> &'static str {
                    #name_literal
                }
                fn inflection() -> Inflection {
                    #inflection
                }
                fn enum_type() -> EnumType {
                    #enum_type
                }
                fn cases() -> Vec<EnumCase> {
                    vec![
                        #cases
                    ]
                }
                fn rust() -> String {
                    #rust.to_string()
                }
            }

        }
    }
}

impl RustTypeEmitter for EnumDef {
    fn ident(&self) -> &Ident {
        &self.ident
    }
    fn tokens(&self) -> &TokenStream {
        &self.tokens
    }
}

#[derive(Default, Clone, Debug)]
pub struct EnumAttr {
    // pub rename_all: Option<Inflection>,
    // pub rename: Option<String>,
    // pub export_to: Option<String>,
    // pub export: bool,
    tag: Option<String>,
    // untagged: bool,
    content: Option<String>,
}

#[derive(Default)]
pub struct SerdeEnumAttr(EnumAttr);

impl EnumAttr {
    // pub fn tagged(&self) -> Result<Tagged<'_>> {
    //     match (false, &self.tag, &self.content) {
    //         (false, None, None) => Ok(Tagged::Externally),
    //         (false, Some(tag), None) => Ok(Tagged::Internally { tag }),
    //         (false, Some(tag), Some(content)) => Ok(Tagged::Adjacently { tag, content }),
    //         (true, None, None) => Ok(Tagged::Untagged),
    //         (true, Some(_), None) => syn_err!("untagged cannot be used with tag"),
    //         (true, _, Some(_)) => syn_err!("untagged cannot be used with content"),
    //         (false, None, Some(_)) => syn_err!("content cannot be used without tag"),
    //     }
    // }

    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut result = Self::default();
        parse_attrs(attrs)?.for_each(|a| result.merge(a));
        // #[cfg(feature = "serde-compat")]
        parse_serde_attrs::<SerdeEnumAttr>(attrs).for_each(|a| result.merge(a.0));
        Ok(result)
    }

    fn merge(
        &mut self,
        EnumAttr {
            // rename_all,
            // rename,
            tag,
            content,
            // untagged,
            // export_to,
            // export,
        }: EnumAttr,
    ) {
        // self.rename = self.rename.take().or(rename);
        // self.rename_all = self.rename_all.take().or(rename_all);
        self.tag = self.tag.take().or(tag);
        // self.untagged = self.untagged || untagged;
        self.content = self.content.take().or(content);
        // self.export = self.export || export;
        // self.export_to = self.export_to.take().or(export_to);
    }
}

impl_parse! {
    EnumAttr(_input, _out) {
        // "rename" => out.rename = Some(parse_assign_str(input)?),
        // "rename_all" => out.rename_all = Some(parse_assign_inflection(input)?),
        // "export_to" => out.export_to = Some(parse_assign_str(input)?),
        // "export" => out.export = true
    }
}

impl_parse! {
    SerdeEnumAttr(input, out) {
        // "rename" => out.0.rename = Some(parse_assign_str(input)?),
        // "rename_all" => out.0.rename_all = Some(parse_assign_inflection(input)?),
        "tag" => out.0.tag = Some(parse_assign_str(input)?),
        "content" => out.0.content = Some(parse_assign_str(input)?),
        // "untagged" => out.0.untagged = true
    }
}
