#![feature(specialization)]
#![macro_use]
// #![deny(unused)]

use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Item, Result};

mod type_def;
use type_def::*;

mod export_types_impl;
use export_types_impl::*;

#[macro_use]
mod utils;

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

    // Access the attributes of the input item

    let (type_def, _ident, _generics) = match input {
        Item::Struct(s) => {
            // println!("Parsed Item::Struct: {:#?}", s);
            (TypeDef::struct_def(&s)?, s.ident, s.generics)
        }
        Item::Enum(e) => {
            // println!("Parsed Item::Enum: {:#?}", e);
            (TypeDef::enum_def(&e)?, e.ident, e.generics)
        }
        _ => {
            syn_err!(input.span(); "unsupported item")
        }
    };

    println!("Type Def Parsed: {:#?}", type_def);
    println!("Type Def Emits: \n{}", type_def.emit());

    Ok(type_def.emit())

    // Ok(ts.into_impl(ident, generics))
}

#[proc_macro]
pub fn export_types(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match export_types_impl(input) {
        Err(err) => err.to_compile_error(),
        Ok(result) => result,
    }
    .into()
}
