use super::syn_type_utils::SynTypeBridge;
use super::type_utils::TypeBridge;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemType, Result};
use type_reflect_core::Type;

#[derive(Clone, Debug)]
pub struct TypeAliasDef {
    pub tokens: TokenStream,
    pub ident: Ident,
    source_type: Type,
}

impl TypeAliasDef {
    pub fn new(item: &ItemType) -> Result<Self> {
        Ok(Self {
            tokens: quote! { #item },
            ident: item.ident.clone(),
            source_type: (item.ty).to_type()?,
        })
    }

    pub fn emit(&self) -> TokenStream {
        let ident = &self.ident;
        let name_literal = format!("{}", ident);
        let rust = format!("{}", self.tokens);
        let type_ = self.source_type.emit_type();

        quote! {
            impl Emittable for #ident {
                fn emit_with<E: TypeEmitter>(emitter: &mut E) -> String {
                    emitter.emit_alias::<Self>()
                }
            }

            impl AliasType for #ident {
                fn name() -> &'static str {
                    #name_literal
                }
                fn source_type() -> Type {
                    #type_
                }
                fn rust() -> String {
                    #rust.to_string()
                }
            }
        }
    }
}
