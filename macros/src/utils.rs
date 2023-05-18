use std::convert::TryFrom;

use proc_macro2::Ident;
use syn::{Attribute, Error, Result};

#[derive(Copy, Clone, Debug)]
pub enum Inflection {
    Lower,
    Upper,
    Camel,
    Snake,
    Pascal,
    ScreamingSnake,
    Kebab,
}

impl Inflection {
    pub fn apply(self, string: &str) -> String {
        use inflector::Inflector;

        match self {
            Inflection::Lower => string.to_lowercase(),
            Inflection::Upper => string.to_uppercase(),
            Inflection::Camel => string.to_camel_case(),
            Inflection::Snake => string.to_snake_case(),
            Inflection::Pascal => string.to_pascal_case(),
            Inflection::ScreamingSnake => string.to_screaming_snake_case(),
            Inflection::Kebab => string.to_kebab_case(),
        }
    }
}

impl TryFrom<String> for Inflection {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Ok(
            match &*value.to_lowercase().replace("_", "").replace("-", "") {
                "lowercase" => Self::Lower,
                "uppercase" => Self::Upper,
                "camelcase" => Self::Camel,
                "snakecase" => Self::Snake,
                "pascalcase" => Self::Pascal,
                "screamingsnakecase" => Self::ScreamingSnake,
                "kebabcase" => Self::Kebab,
                _ => syn_err!("invalid inflection: '{}'", value),
            },
        )
    }
}

/// Converts a rust identifier to a typescript identifier.
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

/// Parse all `#[ts(..)]` attributes from the given slice.
pub fn parse_attrs<'a, A>(attrs: &'a [Attribute]) -> Result<impl Iterator<Item = A>>
where
    A: TryFrom<&'a Attribute, Error = Error>,
{
    Ok(attrs
        .iter()
        .filter(|a| a.path.is_ident("ts"))
        .map(A::try_from)
        .collect::<Result<Vec<A>>>()?
        .into_iter())
}

/// Parse all `#[serde(..)]` attributes from the given slice.
#[cfg(feature = "serde-compat")]
#[allow(unused)]
pub fn parse_serde_attrs<'a, A: TryFrom<&'a Attribute, Error = Error>>(
    attrs: &'a [Attribute],
) -> impl Iterator<Item = A> {
    attrs
        .iter()
        .filter(|a| a.path.is_ident("serde"))
        .flat_map(|attr| match A::try_from(attr) {
            Ok(attr) => Some(attr),
            Err(_) => {
                use quote::ToTokens;
                warning::print_warning(
                    "failed to parse serde attribute",
                    format!("{}", attr.to_token_stream()),
                    "ts-rs failed to parse this attribute. It will be ignored.",
                )
                .unwrap();
                None
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
}

#[cfg(feature = "serde-compat")]
mod warning {
    use std::{fmt::Display, io::Write};

    use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

    // Sadly, it is impossible to raise a warning in a proc macro.
    // This function prints a message which looks like a compiler warning.
    pub fn print_warning(
        title: impl Display,
        content: impl Display,
        note: impl Display,
    ) -> std::io::Result<()> {
        let make_color = |color: Color, bold: bool| {
            let mut spec = ColorSpec::new();
            spec.set_fg(Some(color)).set_bold(bold).set_intense(true);
            spec
        };

        let yellow_bold = make_color(Color::Yellow, true);
        let white_bold = make_color(Color::White, true);
        let white = make_color(Color::White, false);
        let blue = make_color(Color::Blue, true);

        let writer = BufferWriter::stderr(ColorChoice::Auto);
        let mut buffer = writer.buffer();

        buffer.set_color(&yellow_bold)?;
        write!(&mut buffer, "warning")?;
        buffer.set_color(&white_bold)?;
        writeln!(&mut buffer, ": {}", title)?;

        buffer.set_color(&blue)?;
        writeln!(&mut buffer, "  | ")?;

        write!(&mut buffer, "  | ")?;
        buffer.set_color(&white)?;
        writeln!(&mut buffer, "{}", content)?;

        buffer.set_color(&blue)?;
        writeln!(&mut buffer, "  | ")?;

        write!(&mut buffer, "  = ")?;
        buffer.set_color(&white_bold)?;
        write!(&mut buffer, "note: ")?;
        buffer.set_color(&white)?;
        writeln!(&mut buffer, "{}", note)?;

        writer.print(&buffer)
    }
}
