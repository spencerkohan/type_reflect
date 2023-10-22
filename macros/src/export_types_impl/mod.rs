use std::ops::Deref;

use proc_macro2::*;
use quote::*;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::*;
use syn::token::{Bracket, Paren};
use syn::*;

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
enum DestinationArg {
    Dest(Expr),
    Prefix(Prefix),
}

impl Parse for DestinationArg {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Ident) {
            let forked = input.fork();
            let _ident: Ident = forked.parse()?;
            if forked.parse::<Token![:]>().is_ok() && !forked.lookahead1().peek(Ident) {
                // We are fairly certain it's a KeyValuePair now
                let prefix = input.parse::<Prefix>()?;
                return Ok(DestinationArg::Prefix(prefix));
            }
        }
        let expr: Expr = input.parse()?;
        Ok(DestinationArg::Dest(expr))
    }
}

#[derive(Debug, Clone)]
struct Destination {
    export_type: Expr,
    destinations: Vec<Expr>,
    prefix: Option<Expr>,
}

impl Parse for Destination {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut export_type_tokens: TokenStream = quote! {};

        while !input.peek(syn::token::Paren) && !input.is_empty() {
            let next: TokenTree = input.parse()?;
            export_type_tokens.append(next);
        }

        let export_type: Expr = syn::parse2(export_type_tokens)?;

        let content;
        let _parens: Paren = parenthesized!(content in input);

        let mut args: Vec<DestinationArg> = vec![];

        while !content.is_empty() {
            let arg: DestinationArg = content.parse()?;
            args.push(arg);
            if content.peek(Token![,]) {
                let _comma: Token![,] = content.parse()?;
            }
        }

        let mut prefixes: Vec<Prefix> = vec![];

        let destinations: Vec<Expr> = args
            .into_iter()
            .filter_map(|arg| match arg {
                DestinationArg::Dest(expr) => Some(expr),
                DestinationArg::Prefix(pref) => {
                    prefixes.push(pref);
                    None
                }
            })
            .collect();

        if prefixes.len() > 1 {
            let pref = &prefixes[1];
            return Err(syn::Error::new(
                pref.ident.span(),
                r#"Only a single "prefix" argument is allowed in a destination declaration"#,
            ));
        }

        let prefix: Option<Expr> = prefixes.first().map(|pref| pref.expr.clone());

        Ok(Self {
            export_type,
            destinations,
            prefix,
        })
    }
}

#[derive(Debug, Clone)]
struct Prefix {
    ident: Ident,
    expr: Expr,
}

impl Parse for Prefix {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;

        if ident.to_string().as_str() != "prefix" {
            return Err(syn::Error::new(
                ident.span(),
                r#"Expected the argument name "prefix" when parsing a prefix"#,
            ));
        }

        let _colon_token: Token![:] = input.parse()?;
        let expr = input.parse()?;

        Ok(Self { ident, expr })
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

fn emit_destination(dest: &Destination, types: &Vec<&Ident>) -> TokenStream {
    let emitter = &dest.export_type;

    let prefix = match &dest.prefix {
        Some(expr) => {
            quote! { #expr }
        }
        None => quote! { "" },
    };

    let mut result = quote! {};
    for dest in &dest.destinations {
        result.extend(quote! {
            let mut file = #emitter::init_destination_file(#dest, #prefix)?;
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
