use super::syn_type_utils::*;
use super::type_utils::*;
use super::RustTypeEmitter;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Field, FieldsNamed, ItemStruct, Result};
use type_reflect_core::type_description::StructMember;

#[derive(Clone, Debug)]
pub struct StructDef {
    tokens: TokenStream,
    ident: Ident,
    members: Vec<StructMember>,
}

fn extract_members(item: &ItemStruct) -> Result<Vec<StructMember>> {
    match &(item.fields) {
        syn::Fields::Named(fields) => (&fields).to_struct_members(),
        syn::Fields::Unnamed(_) => todo!(),
        syn::Fields::Unit => todo!(),
    }
}

impl StructDef {
    pub fn new(item: &ItemStruct) -> Result<Self> {
        Ok(Self {
            tokens: quote! { #item },
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
        quote! {

            impl Emittable for #ident {
                fn emit_with<E: TypeEmitter>() -> String {
                    E::emit_struct::<Self>()
                }
            }

            impl StructType for #ident {
                fn name() -> &'static str {
                    #name_literal
                }
                fn members() -> Vec<StructMember> {
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