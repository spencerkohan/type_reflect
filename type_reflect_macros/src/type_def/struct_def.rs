use super::syn_type_utils::*;
use super::type_utils::*;
use super::InflectionTokenProvider;
use super::RustTypeEmitter;
use crate::attribute_utils::RenameAllAttr;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{ItemStruct, Result};
use type_reflect_core::{type_description::NamedField, Inflection};

#[derive(Clone, Debug)]
pub struct StructDef {
    tokens: TokenStream,
    inflection: Inflection,
    ident: Ident,
    members: Vec<NamedField>,
}

fn extract_members(item: &ItemStruct) -> Result<Vec<NamedField>> {
    match &(item.fields) {
        syn::Fields::Named(fields) => (&fields).to_struct_members(),
        syn::Fields::Unnamed(fieldsUnnamed) => todo!(),
        syn::Fields::Unit => todo!(),
    }
}

impl StructDef {
    pub fn new(item: &ItemStruct) -> Result<Self> {
        let rename_attr = RenameAllAttr::from_attrs(&item.attrs)?;

        Ok(Self {
            tokens: quote! { #item },
            inflection: rename_attr.rename_all,
            ident: item.ident.clone(),
            members: extract_members(&item)?,
        })
    }

    pub fn emit_members(&self) -> TokenStream {
        let members: Vec<TokenStream> = (&self.members)
            .into_iter()
            .map(|member| member.emit_member())
            .collect();
        quote! {
            #(#members),*
        }
    }

    pub fn emit(&self) -> TokenStream {
        let ident = &self.ident();
        let name_literal = format!("{}", ident);
        let members = &self.emit_members();
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
                fn fields() -> Vec<NamedField> {
                    vec![
                        #members
                    ]
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
