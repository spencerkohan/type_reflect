use crate::attribute_utils::*;
use crate::type_def::InflectionTokenProvider;
// use crate::utils::*;
use type_reflect_core::EnumType;
use type_reflect_core::Inflection;

use super::{syn_type_utils::*, type_utils::TypeFieldsDefinitionBridge, RustTypeEmitter};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, ItemEnum, Result};
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
            let attrs = RenameAllAttr::from_attrs(&case.attrs);
            Ok(EnumCase {
                name,
                type_: (&case.fields).to_fields()?,
                inflection: attrs.rename_all,
                rename: attrs.rename,
            })
        })
        .collect()
}

impl EnumDef {
    pub fn new(item: &ItemEnum) -> Result<Self> {
        let attributes = EnumAttr::from_attrs(&item.attrs);
        let rename_attr = RenameAllAttr::from_attrs(&item.attrs);

        let cases = extract_cases(&item)?;

        let enum_type = match (&cases).into_iter().fold(false, |input, case| {
            input
                || if let TypeFieldsDefinition::Unit = case.type_ {
                    false
                } else {
                    true
                }
        }) {
            // false indicates it is not complex
            false => EnumType::Simple,
            // true indicates the type is complex
            true => {
                if attributes.untagged {
                    EnumType::Untagged
                } else {
                    match attributes.tag {
                        Some(case_key) => {
                            let content_key = attributes.content;
                            EnumType::Complex {
                                case_key,
                                content_key,
                            }
                        }
                        None => EnumType::ExternallyTagged,
                    }
                }
            },
        };

        // `#[serde(tag = ...)]` (internally tagged) enums can only represent
        // tuple variants through the `content` key. Serde rejects
        // multi-field tuples at compile time and panics at runtime for
        // newtypes, so reject the shape here, at the variant, with an
        // actionable message.
        if let EnumType::Complex {
            content_key: None, ..
        } = &enum_type
        {
            for variant in &item.variants {
                if let syn::Fields::Unnamed(_) = variant.fields {
                    return Err(syn::Error::new(
                        variant.ident.span(),
                        "tuple variants in a tagged enum require a `content` key, e.g. #[serde(tag = \"_case\", content = \"_\")].",
                    ));
                }
            }
        }

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
                let type_ = case.type_.emit_def();
                let rename_all = &case.inflection.to_tokens();
                let rename = match &case.rename {
                    Some(rename) => quote! { Some(#rename.to_string()) },
                    None => quote! { None },
                };
                quote! {
                    EnumCase {
                        name: #name.to_string(),
                        type_: #type_,
                        inflection: #rename_all,
                        rename: #rename,
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
                    EnumType::Complex {
                        case_key: #case_key.to_string(),
                        content_key: Some(#content_key.to_string())
                    }
                },
                None => quote! {
                    EnumType::Complex { case_key: #case_key.to_string(), content_key: None }
                },
            },
            EnumType::ExternallyTagged => quote! { EnumType::ExternallyTagged },
            EnumType::Untagged => quote! { EnumType::Untagged },
        };

        let inflection = &self.inflection.to_tokens();

        quote! {

            impl Emittable for #ident {
                fn emit_with<E: TypeEmitter>(emitter: &mut E) -> String {
                    emitter.emit_enum::<Self>()
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
    tag: Option<String>,
    content: Option<String>,
    untagged: bool,
}

impl EnumAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Self {
        parse_serde_attrs::<EnumAttr>(attrs).fold(Self::default(), |mut result, a| {
            result.merge(a);
            result
        })
    }

    fn merge(&mut self, EnumAttr { tag, content, untagged }: EnumAttr) {
        self.tag = self.tag.take().or(tag);
        self.content = self.content.take().or(content);
        self.untagged = self.untagged || untagged;
    }
}

impl_parse! {
    EnumAttr(input, out) {
        "tag" => out.tag = Some(parse_assign_str(input)?),
        "content" => out.content = Some(parse_assign_str(input)?),
        "untagged" => out.untagged = true,
    }
}
