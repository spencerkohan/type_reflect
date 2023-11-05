use proc_macro2::*;
use quote::*;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::*;
use syn::token::{Bracket, Paren};
use syn::*;
mod destination;
use destination::*;

#[derive(Debug, Clone)]
struct ItemsList {
    idents: Punctuated<Ident, Token![,]>,
}

impl ItemsList {
    fn args(&self) -> Vec<&Ident> {
        (&self.idents).into_iter().collect()
    }
}

impl Parse for ItemsList {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        if ident.to_string().as_str() != "types" {
            return Err(syn::Error::new(
                ident.span(),
                r#"Expected argument name: "types""#,
            ));
        }
        let _colon_token: Token![:] = input.parse()?;
        let content;
        let _brackets: Bracket = bracketed!(content in input);
        let idents = content.parse_terminated(Ident::parse)?;
        Ok(Self { idents })
    }
}

#[derive(Debug, Clone)]
struct DestinationList {
    destinations: Vec<Destination>,
}

impl Parse for DestinationList {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        if ident.to_string().as_str() != "destinations" {
            return Err(syn::Error::new(
                ident.span(),
                r#"Expected argument name: "destinations""#,
            ));
        }

        let _colon_token: Token![:] = input.parse()?;
        let content;
        let _brackets: Bracket = bracketed!(content in input);
        let destinations: Punctuated<Destination, Token![,]> =
            match content.parse_terminated(Destination::parse) {
                Ok(res) => res,
                Err(err) => {
                    return Err(syn::Error::new(
                        err.span(),
                        format!("Error parsing destinations list: {}", err),
                    ));
                }
            };

        let destinations: Vec<Destination> = destinations.into_iter().map(|dest| dest).collect();

        Ok(Self { destinations })
    }
}

#[derive(Debug, Clone)]
pub struct NamedArg {
    ident: Ident,
    expr: Expr,
}

impl NamedArg {
    pub fn name(&self) -> String {
        self.ident.to_string()
    }
}

impl ToTokens for NamedArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let expr = &self.expr;
        tokens.extend(quote! { #ident: #expr })
    }
}

impl Parse for NamedArg {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;

        let _colon_token: Token![:] = input.parse()?;
        let expr: Expr = input.parse()?;

        Ok(Self { ident, expr })
    }
}

#[derive(Debug, Clone)]
pub enum DestinationArg {
    Dest(Expr),
    Named(NamedArg),
}

pub fn peak_arg_name(input: &syn::parse::ParseStream) -> Option<Ident> {
    let lookahead = input.lookahead1();
    if lookahead.peek(Ident) {
        let forked = input.fork();
        let ident: Ident = forked.parse().unwrap();
        if forked.parse::<Token![:]>().is_ok() && !forked.lookahead1().peek(Ident) {
            // We are fairly certain it's a KeyValuePair now
            return Some(ident);
        }
    }
    None
}

impl Parse for DestinationArg {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Ident) {
            let forked = input.fork();
            let _ident: Ident = forked.parse()?;
            if forked.parse::<Token![:]>().is_ok() && !forked.lookahead1().peek(Ident) {
                // We are fairly certain it's a KeyValuePair now
                let prefix = input.parse::<NamedArg>()?;
                return Ok(DestinationArg::Named(prefix));
            }
        }
        let expr: Expr = input.parse()?;
        Ok(DestinationArg::Dest(expr))
    }
}

#[derive(Debug, Clone)]
struct Input {
    items: ItemsList,
    destinations: DestinationList,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let items = input.parse()?;
        let _comma_token: Token![,] = input.parse()?;
        let destinations = input.parse()?;
        Ok(Self {
            items,
            destinations,
        })
    }
}

pub fn export_types_impl(input: proc_macro::TokenStream) -> Result<TokenStream> {
    // println!("EXPORT TYPES input: {:#?}", input);
    let input = syn::parse::<Input>(input)?;
    // println!("parse result: {:#?}", input);

    let types = input.items.args();
    let destinations = input.destinations.destinations;

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
