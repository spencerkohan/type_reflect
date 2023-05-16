use super::RustTypeEmitter;
use proc_macro2::{Ident, TokenStream};
use quote::{quote};
use syn::{
    ItemEnum,
};

#[derive(Clone, Debug)]
pub struct EnumDef {
    tokens: TokenStream,
    ident: Ident,
}

impl EnumDef {
    pub fn new(item: &ItemEnum) -> Self {
        Self {
            tokens: quote! { #item },
            ident: item.ident.clone(),
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
