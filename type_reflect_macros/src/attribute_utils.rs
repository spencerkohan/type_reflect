use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Ident, Lit, Result, Token};
pub use type_reflect_core::inflection::*;
use type_reflect_core::impl_parse;

#[derive(Default, Clone, Debug)]
pub struct RenameAllAttr {
    pub rename_all: Inflection,
}

impl RenameAllAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Self {
        parse_serde_attrs::<RenameAllAttr>(attrs).fold(Self::default(), |mut result, a| {
            result.merge(a);
            result
        })
    }

    fn merge(&mut self, RenameAllAttr { rename_all }: RenameAllAttr) {
        self.rename_all = rename_all;
    }
}

impl_parse! {
    RenameAllAttr(input, out) {
        "rename_all" => out.rename_all = parse_assign_inflection(input)?,
    }
}

/// Parse all `#[serde(..)]` attributes from the given slice.
///
/// Parsing is lenient: unknown keys are skipped (see `impl_parse!`), so an
/// attribute only fails on malformed syntax, and a failed attribute is
/// dropped rather than aborting the derive.
#[allow(unused)]
pub fn parse_serde_attrs<'a, A: TryFrom<&'a Attribute, Error = syn::Error>>(
    attrs: &'a [Attribute],
) -> impl Iterator<Item = A> {
    attrs
        .iter()
        .filter(|a| a.path.is_ident("serde"))
        .flat_map(|attr| A::try_from(attr).ok())
        .collect::<Vec<_>>()
        .into_iter()
}

pub fn parse_assign_str(input: ParseStream) -> Result<String> {
    input.parse::<Token![=]>()?;
    match Lit::parse(input)? {
        Lit::Str(string) => Ok(string.value()),
        other => Err(syn::Error::new(other.span(), "expected string")),
    }
}

pub fn parse_assign_inflection(input: ParseStream) -> Result<Inflection> {
    match parse_assign_str(input) {
        Ok(str) => Inflection::try_from(str),
        Err(_) => Ok(Inflection::None),
    }
}
