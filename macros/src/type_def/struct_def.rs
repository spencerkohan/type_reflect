use super::syn_type_utils::SynTypeBridge;
use super::type_utils::*;
use super::RustTypeEmitter;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Field, ItemStruct, Result};
use type_reflect_core::type_description::StructMember;

#[derive(Clone, Debug)]
pub struct StructDef {
    tokens: TokenStream,
    ident: Ident,
    members: Vec<StructMember>,
}

fn get_struct_member(field: &Field) -> Result<StructMember> {
    println!("Getting struct member from field: {:#?}", field);
    let name = match &field.ident {
        None => panic!("Struct fields must be named: {:#?}", field),
        Some(ident) => format!("{}", ident),
    };

    let type_ = field.ty.to_type()?;

    Ok(StructMember { name, type_ })
}

fn extract_members(item: &ItemStruct) -> Result<Vec<StructMember>> {
    match &(item.fields) {
        syn::Fields::Named(fields) => (&fields.named)
            .into_iter()
            .map(|field: &Field| get_struct_member(&field))
            .collect(),
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
            .map(|member| {
                let name = &member.name;
                let type_ = member.type_.emit_type();
                quote! {
                    StructMember {
                        name: #name.to_string(),
                        type_: #type_,
                    }
                }
            })
            .collect();
        quote! {
            #(#members),*
        }
    }

    pub fn emit(&self) -> TokenStream {
        let ident = &self.ident();
        let name_literal = format!("{}", ident);
        let members = &self.emit_members();
        quote! {
            impl StructType for #ident {
                fn name() -> &'static str {
                    #name_literal
                }
                fn members() -> Vec<StructMember> {
                    vec![
                        #members
                    ]
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
