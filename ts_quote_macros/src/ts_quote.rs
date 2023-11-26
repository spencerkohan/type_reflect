use crate::parsing::ParseContext;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

pub fn macro_impl(input: TokenStream) -> Result<TokenStream> {
    let mut parse_context = ParseContext::new("0".to_string(), input);
    parse_context.parse();

    let raw_string = &parse_context.format_string();
    let substitution_mappings = parse_context.substitution_mappings();

    Ok(quote! {
        ts_quote::TS::from_source(format!(#raw_string, #substitution_mappings))
    })
}
