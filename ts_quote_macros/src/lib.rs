// #![allow(incomplete_features)]
// #![feature(specialization)]
#![macro_use]

mod parsing;
mod ts_quote;
mod ts_string;

/**
ts_string is a utility macro for emitting typescript strings from rust code

usage:

let ts: String = ts_string!{
const x = 7;
};
assert_eq!(ts, "const x = 7;".to_string());

**/
#[proc_macro]
pub fn ts_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match ts_string::macro_impl(input.into()) {
        Err(err) => err.to_compile_error(),
        Ok(result) => result,
    }
    .into()
}

/**
ts_quote is a utility macro for emitting typescript from rust code

ts_quote returns a Result<deno_ast::ParsedSource, deno_ast::Diagnostic>

This is aliased to the ts_quote::TS type

usage:

let ts: TS = ts_quote!{
const x = 7;
}?;
assert_eq!(ts.formatted(None), "const x = 7;".to_string());

**/
#[proc_macro]
pub fn ts_quote(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match ts_quote::macro_impl(input.into()) {
        Err(err) => err.to_compile_error(),
        Ok(result) => result,
    }
    .into()
}
