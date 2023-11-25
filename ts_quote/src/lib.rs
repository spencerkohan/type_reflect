// #![allow(incomplete_features)]
// #![feature(specialization)]
#![macro_use]

mod parsing;

mod ts_string;

/**
ts_str is a utility macro for emitting typescript strings from rust code

usage:


let ts = ts_str!{
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
