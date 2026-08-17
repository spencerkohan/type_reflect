use proc_macro2::Ident;

/// Converts a rust identifier to a typescript identifier.
#[allow(unused)]
pub fn to_ts_ident(ident: &Ident) -> String {
    let ident = ident.to_string();
    if ident.starts_with("r#") {
        ident.trim_start_matches("r#").to_owned()
    } else {
        ident
    }
}


