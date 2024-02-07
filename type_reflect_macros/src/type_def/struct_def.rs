use super::syn_type_utils::*;
use super::type_utils::*;
use super::InflectionTokenProvider;
use super::RustTypeEmitter;
use crate::attribute_utils::RenameAllAttr;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{ItemStruct, Result};
use type_reflect_core::Inflection;
use type_reflect_core::TypeFieldsDefinition;

#[derive(Clone, Debug)]
pub struct StructDef {
    tokens: TokenStream,
    inflection: Inflection,
    ident: Ident,
    fields: TypeFieldsDefinition,
}

// fn extract_members(item: &ItemStruct) -> Result<TypeFieldsDefinition> {
//     match &(item.fields) {
//         syn::Fields::Named(fields) => (&fields).to_named_fields(),
//         syn::Fields::Unnamed(fieldsUnnamed) => todo!(),
//         syn::Fields::Unit => todo!(),
//     }
// }

impl StructDef {
    pub fn new(item: &ItemStruct) -> Result<Self> {
        let rename_attr = RenameAllAttr::from_attrs(&item.attrs)?;
        Ok(Self {
            tokens: quote! { #item },
            inflection: rename_attr.rename_all,
            ident: item.ident.clone(),
            fields: (&item.fields).to_fields()?,
        })
    }

    pub fn emit_fields(&self) -> TokenStream {
        // let members: Vec<TokenStream> = (&self.fields)
        //     .into_iter()
        //     .map(|member| member.emit_member())
        //     .collect();
        // quote! {
        //     #(#members),*
        // }

        return (&self.fields).emit_def();
    }

    pub fn emit(&self) -> TokenStream {
        let ident = &self.ident();
        let name_literal = format!("{}", ident);
        let members = &self.emit_fields();
        let rust = format!("{}", self.tokens());
        let inflection = &self.inflection.to_tokens();
        quote! {

            impl Emittable for #ident {
                fn emit_with<E: TypeEmitter>(emitter: &mut E) -> String {
                    emitter.emit_struct::<Self>()
                }
            }

            impl StructType for #ident {
                fn name() -> &'static str {
                    #name_literal
                }
                fn inflection() -> Inflection {
                    #inflection
                }
                fn fields() -> TypeFieldsDefinition {
                    #members
                }
                fn rust() -> String {
                    #rust.to_string()
                }
            }
        }
    }
}

impl RustTypeEmitter for StructDef {
    fn ident(&self) -> &Ident {
        &self.ident
    }
    fn tokens(&self) -> &TokenStream {
        &self.tokens
    }
}
