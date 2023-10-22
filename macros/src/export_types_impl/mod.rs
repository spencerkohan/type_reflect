use std::ops::Deref;

use proc_macro2::*;
use quote::*;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::*;
use syn::token::Bracket;
use syn::*;

#[derive(Debug, Clone)]
struct ItemsList {
    ident: Ident,
    colon_token: Token![:],
    brackets: Bracket,
    idents: Punctuated<Ident, Token![,]>,
}

impl ItemsList {
    fn args(&self) -> Vec<&Ident> {
        (&self.idents).into_iter().collect()
    }
}

impl Parse for ItemsList {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse()?;
        let colon_token = input.parse()?;
        let content;
        let brackets = bracketed!(content in input);
        let idents = content.parse_terminated(Ident::parse)?;
        Ok(Self {
            ident,
            colon_token,
            brackets,
            idents,
        })
    }
}

#[derive(Debug, Clone)]
struct DestinationList {
    ident: Ident,
    colon_token: Token![:],
    brackets: Bracket,
    destinations: Punctuated<ExprCall, Token![,]>,
}

impl Parse for DestinationList {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse()?;
        let colon_token = input.parse()?;
        let content;
        let brackets = bracketed!(content in input);
        let destinations = content.parse_terminated(ExprCall::parse)?;
        Ok(Self {
            ident,
            colon_token,
            brackets,
            destinations,
        })
    }
}

struct Destination {
    export_type: Expr,
    destinations: Vec<Expr>,
}

impl DestinationList {
    fn args(&self) -> Vec<Destination> {
        (&self.destinations)
            .into_iter()
            .map(|dest| Destination {
                export_type: dest.func.deref().clone(),
                destinations: (&dest.args).into_iter().map(|arg| arg.clone()).collect(),
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Input {
    items: ItemsList,
    comma_token: Token![,],
    destinations: DestinationList,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let items = input.parse()?;
        let comma_token = input.parse()?;
        let destinations = input.parse()?;
        Ok(Self {
            items,
            comma_token,
            destinations,
        })
    }
}

fn emit_destination(dest: &Destination, types: &Vec<&Ident>) -> TokenStream {
    let emitter = &dest.export_type;

    let mut result = quote! {};
    for dest in &dest.destinations {
        result.extend(quote! {
            let mut file = #emitter::init_destination_file(#dest)?;
        });
        for type_ in types {
            result.extend(quote! {
                file.write_all(#emitter::emit::<#type_>().as_bytes())?;
            });
        }
        result.extend(quote! {
            #emitter::finalize(#dest)?;
        });
    }
    result
}

pub fn export_types_impl(input: proc_macro::TokenStream) -> Result<TokenStream> {
    // println!("EXPORT TYPES input: {:#?}", input);
    let input = syn::parse::<Input>(input)?;
    // println!("parse result: {:#?}", input);

    let types = input.items.args();
    let destinations = input.destinations.args();

    let mut result = quote! {};
    for dest in destinations {
        result.extend(emit_destination(&dest, &types))
    }

    let result = quote! {
        (|| -> Result<(), std::io::Error> {
            #result
            Ok(())
        })()
    };

    // println!("Emitting: {}", result);
    // Ok(input)
    Ok(result)
}
