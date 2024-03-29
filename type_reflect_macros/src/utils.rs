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

// /// Parse all `#[ts(..)]` attributes from the given slice.
// pub fn parse_attrs<'a, A>(attrs: &'a [Attribute]) -> Result<impl Iterator<Item = A>>
// where
//     A: TryFrom<&'a Attribute, Error = Error>,
// {
//     Ok(attrs
//         .iter()
//         .filter(|a| a.path.is_ident("ts"))
//         .map(A::try_from)
//         .collect::<Result<Vec<A>>>()?
//         .into_iter())
// }

// /// Parse all `#[serde(..)]` attributes from the given slice.
// #[cfg(feature = "serde-compat")]
// #[allow(unused)]
// pub fn parse_serde_attrs<'a, A: TryFrom<&'a Attribute, Error = Error>>(
//     attrs: &'a [Attribute],
// ) -> impl Iterator<Item = A> {
//     attrs
//         .iter()
//         .filter(|a| a.path.is_ident("serde"))
//         .flat_map(|attr| match A::try_from(attr) {
//             Ok(attr) => Some(attr),
//             Err(_) => {
//                 use quote::ToTokens;
//                 warning::print_warning(
//                     "failed to parse serde attribute",
//                     format!("{}", attr.to_token_stream()),
//                     "ts-rs failed to parse this attribute. It will be ignored.",
//                 )
//                 .unwrap();
//                 None
//             }
//         })
//         .collect::<Vec<_>>()
//         .into_iter()
// }

// #[cfg(feature = "serde-compat")]
// mod warning {
//     use std::{fmt::Display, io::Write};

//     use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

//     // Sadly, it is impossible to raise a warning in a proc macro.
//     // This function prints a message which looks like a compiler warning.
//     pub fn print_warning(
//         title: impl Display,
//         content: impl Display,
//         note: impl Display,
//     ) -> std::io::Result<()> {
//         let make_color = |color: Color, bold: bool| {
//             let mut spec = ColorSpec::new();
//             spec.set_fg(Some(color)).set_bold(bold).set_intense(true);
//             spec
//         };

//         let yellow_bold = make_color(Color::Yellow, true);
//         let white_bold = make_color(Color::White, true);
//         let white = make_color(Color::White, false);
//         let blue = make_color(Color::Blue, true);

//         let writer = BufferWriter::stderr(ColorChoice::Auto);
//         let mut buffer = writer.buffer();

//         buffer.set_color(&yellow_bold)?;
//         write!(&mut buffer, "warning")?;
//         buffer.set_color(&white_bold)?;
//         writeln!(&mut buffer, ": {}", title)?;

//         buffer.set_color(&blue)?;
//         writeln!(&mut buffer, "  | ")?;

//         write!(&mut buffer, "  | ")?;
//         buffer.set_color(&white)?;
//         writeln!(&mut buffer, "{}", content)?;

//         buffer.set_color(&blue)?;
//         writeln!(&mut buffer, "  | ")?;

//         write!(&mut buffer, "  = ")?;
//         buffer.set_color(&white_bold)?;
//         write!(&mut buffer, "note: ")?;
//         buffer.set_color(&white)?;
//         writeln!(&mut buffer, "{}", note)?;

//         writer.print(&buffer)
//     }
// }
