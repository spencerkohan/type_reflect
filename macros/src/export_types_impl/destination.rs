use proc_macro2::*;
use quote::*;
use syn::parse::{Parse, ParseStream};
use syn::token::Paren;
use syn::*;

use super::{peak_arg_name, DestinationArg, NamedArg};

#[derive(Debug, Clone)]
pub enum Destination {
    Named(NamedDestination),
    Unnamed(UnnamedDestination),
}

impl Parse for Destination {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(syn::token::Paren) {
            let dest = input.parse()?;
            return Ok(Destination::Unnamed(dest));
        }
        let dest = input.parse()?;
        return Ok(Destination::Named(dest));
    }
}

#[derive(Debug, Clone)]
pub struct NamedDestination {
    pub export_type: Expr,
    pub destinations: Vec<Expr>,
    pub named_args: Vec<NamedArg>,
    pub prefix: Option<Expr>,
}

impl Parse for NamedDestination {
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

        let mut named_args: Vec<NamedArg> = vec![];

        let destinations: Vec<Expr> = args
            .into_iter()
            .filter_map(|arg| match arg {
                DestinationArg::Dest(expr) => Some(expr),
                DestinationArg::Named(arg) => {
                    named_args.push(arg);
                    None
                }
            })
            .collect();

        let mut prefix: Option<Expr> = None;
        let named_args = named_args
            .into_iter()
            .filter(|arg| {
                match arg.name().as_str() {
                    "prefix" => {
                        prefix = Some(arg.expr.clone());
                        return false;
                    }
                    _ => {}
                };
                true
            })
            .collect();

        Ok(Self {
            export_type,
            destinations,
            named_args,
            prefix,
        })
    }
}

#[derive(Debug, Clone)]
pub struct EmitterDecl {
    pub type_name: Expr,
    pub args: Vec<NamedArg>,
}

impl Parse for EmitterDecl {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut export_type_tokens: TokenStream = quote! {};

        while !input.peek(syn::token::Paren) && !input.is_empty() {
            let next: TokenTree = input.parse()?;
            export_type_tokens.append(next);
        }

        let type_name: Expr = syn::parse2(export_type_tokens)?;

        eprintln!(
            "Parsing emitter decl: {}",
            type_name.clone().into_token_stream()
        );

        let content;
        let _parens: Paren = parenthesized!(content in input);

        let mut args: Vec<NamedArg> = vec![];

        while !content.is_empty() {
            let arg: NamedArg = content.parse()?;
            args.push(arg);
            if content.peek(Token![,]) {
                let _comma: Token![,] = content.parse()?;
            }
        }

        Ok(Self { type_name, args })
    }
}

#[derive(Debug, Clone)]
pub struct EmitterDeclList {
    pub emitters: Vec<EmitterDecl>,
}

impl Parse for EmitterDeclList {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _parens: token::Bracket = bracketed!(content in input);
        let mut emitters: Vec<EmitterDecl> = vec![];

        while !content.is_empty() {
            let emitter: EmitterDecl = content.parse()?;
            emitters.push(emitter);
            if content.peek(Token![,]) {
                let _comma: Token![,] = content.parse()?;
            }
        }

        Ok(Self { emitters })
    }
}

fn parse_emitters(input: TokenStream) -> Result<Vec<EmitterDecl>> {
    let emitter_decls: EmitterDeclList = syn::parse2::<EmitterDeclList>(input)?;
    Ok(emitter_decls.emitters)
}

#[derive(Debug, Clone)]
pub struct UnnamedDestination {
    pub destinations: Vec<Expr>,
    pub prefix: Option<Expr>,
    pub emitters: Vec<EmitterDecl>,
}

impl Parse for UnnamedDestination {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _parens: Paren = parenthesized!(content in input);

        // let mut args: Vec<DestinationArg> = vec![];

        let mut destinations: Vec<Expr> = vec![];
        let mut prefix: Option<Expr> = None;
        let mut emitters: EmitterDeclList = EmitterDeclList { emitters: vec![] };

        while !content.is_empty() {
            match peak_arg_name(&&content) {
                Some(name) => match name.to_string().as_str() {
                    "prefix" => {
                        prefix = Some(content.parse()?);
                    }
                    "emitters" => {
                        let _: Ident = content.parse()?;
                        let _: Token![:] = content.parse()?;
                        emitters = content.parse()?;
                    }
                    _ => {
                        // TODO: this should produce an error
                    }
                },
                None => {
                    let dest: Expr = content.parse()?;
                    destinations.push(dest);
                }
            }
            if content.peek(Token![,]) {
                let _comma: Token![,] = content.parse()?;
            }
        }

        // let destinations: Vec<Expr> = args
        //     .into_iter()
        //     .filter_map(|arg| match arg {
        //         DestinationArg::Dest(expr) => Some(expr),
        //         DestinationArg::Named(arg) => {
        //             named_args.push(arg);
        //             None
        //         }
        //     })
        //     .collect();

        // // TODO: validate that named args is empty
        // let _named_args: Vec<NamedArg> = named_args
        //     .into_iter()
        //     .filter(|arg| {
        //         match arg.name().as_str() {
        //             "prefix" => {
        //                 prefix = Some(arg.expr.clone());
        //                 return false;
        //             }
        //             "emitters" => {
        //                 emitter_tokens = arg.expr.clone().into_token_stream();
        //                 return false;
        //             }
        //             _ => {}
        //         };
        //         true
        //     })
        //     .collect();

        // let emitters = parse_emitters(emitter_tokens)?;

        Ok(Self {
            destinations,
            prefix,
            emitters: emitters.emitters,
        })
    }
}

pub fn emit_destination(dest: &Destination, types: &Vec<&Ident>) -> TokenStream {
    match dest {
        Destination::Named(dest) => emit_named_destination(dest, types),
        Destination::Unnamed(dest) => emit_unnamed_destination(dest, types),
    }
}

pub fn emit_named_destination(dest: &NamedDestination, types: &Vec<&Ident>) -> TokenStream {
    let emitter = &dest.export_type;

    let prefix = match &dest.prefix {
        Some(expr) => {
            quote! { #expr }
        }
        None => quote! { "" },
    };

    let emitter_args = &dest.named_args;
    let emitter_args = quote! { #(#emitter_args,)* };

    let mut result = quote! {};
    for dest in &dest.destinations {
        result.extend(quote! {
            let mut file = type_reflect::init_destination_file(#dest, #prefix)?;
            let mut emitter = #emitter {
                #emitter_args
                ..Default::default()
            };
            file.write_all(emitter.prefix().as_bytes())?;
        });
        for type_ in types {
            result.extend(quote! {
                file.write_all(emitter.emit::<#type_>().as_bytes())?;
            });
        }
        result.extend(quote! {
            emitter.finalize(#dest)?;
        });
    }
    result
}

pub fn emit_single_emitter(emitter: &EmitterDecl, types: &Vec<&Ident>, dest: &Expr) -> TokenStream {
    let emitter_name = &emitter.type_name;

    let emitter_args = &emitter.args;
    let emitter_args = quote! { #(#emitter_args,)* };

    let mut result = quote! {};
    result.extend(quote! {
        let mut emitter = #emitter_name {
            #emitter_args
            ..Default::default()
        };
        file.write_all(emitter.prefix().as_bytes())?;
    });
    for type_ in types {
        result.extend(quote! {
            file.write_all(emitter.emit::<#type_>().as_bytes())?;
        });
    }
    result.extend(quote! {
        emitter.finalize(#dest)?;
    });

    result
}

pub fn emit_unnamed_destination(dest: &UnnamedDestination, types: &Vec<&Ident>) -> TokenStream {
    let prefix = match &dest.prefix {
        Some(expr) => {
            quote! { #expr }
        }
        None => quote! { "" },
    };

    let emitters = &dest.emitters;

    let mut result = quote! {};
    for dest in &dest.destinations {
        result.extend(quote! {
            let mut file = type_reflect::init_destination_file(#dest, #prefix)?;
        });
        for emitter in emitters {
            result.extend(emit_single_emitter(emitter, types, dest));
        }
    }
    result
}
