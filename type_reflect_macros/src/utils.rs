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

/// Convert an arbitrary name to a valid Typescript field name.
///
/// If the name contains special characters it will be wrapped in quotes.
#[allow(unused)]
pub fn raw_name_to_ts_field(value: String) -> String {
    let valid = value
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '$')
        && value
            .chars()
            .next()
            .map(|first| !first.is_numeric())
            .unwrap_or(true);
    if !valid {
        format!(r#""{value}""#)
    } else {
        value
    }
}

