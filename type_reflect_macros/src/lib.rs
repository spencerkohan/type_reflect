#![macro_use]

use proc_macro2::TokenStream;
use syn::{spanned::Spanned, Item, Result};

mod type_def;
use type_def::*;
mod attribute_utils;

mod export_types_impl;
use export_types_impl::*;
use type_reflect_core::syn_err;

/// Derives [TS](./trait.TS.html) for a struct or enum.
/// Please take a look at [TS](./trait.TS.html) for documentation.
#[proc_macro_derive(Reflect, attributes(reflect))]
pub fn reflect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match entry(input) {
        Err(err) => err.to_compile_error(),
        Ok(result) => result,
    }
    .into()
}

fn entry(input: proc_macro::TokenStream) -> Result<TokenStream> {
    let input = syn::parse::<Item>(input)?;

    let (type_def, _ident, _generics) = match input {
        Item::Struct(s) => (TypeDef::struct_def(&s)?, s.ident, s.generics),
        Item::Enum(e) => (TypeDef::enum_def(&e)?, e.ident, e.generics),
        Item::Type(t) => (TypeDef::alias_def(&t)?, t.ident, t.generics),
        _ => {
            syn_err!(input.span(); "Item is not supported by the Reflect macro")
        }
    };

    Ok(type_def.emit())
}

#[proc_macro]
pub fn export_types(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match export_types_impl(input) {
        Err(err) => err.to_compile_error(),
        Ok(result) => result,
    }
    .into()
}
