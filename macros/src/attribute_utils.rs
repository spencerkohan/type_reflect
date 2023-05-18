use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Ident, Lit, Result, Token};
pub use type_reflect_core::inflection::*;
use type_reflect_core::{impl_parse, syn_err};

#[derive(Default, Clone, Debug)]
pub struct RenameAllAttr {
    pub rename_all: Inflection,
}

impl RenameAllAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut result = Self::default();
        parse_attrs(attrs)?.for_each(|a| result.merge(a));
        parse_serde_attrs::<RenameAllAttr>(attrs).for_each(|a| result.merge(a));
        Ok(result)
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

/// Parse all `#[ts(..)]` attributes from the given slice.
pub fn parse_attrs<'a, A>(attrs: &'a [Attribute]) -> Result<impl Iterator<Item = A>>
where
    A: TryFrom<&'a Attribute, Error = syn::Error>,
{
    Ok(attrs
        .iter()
        .filter(|a| a.path.is_ident("ts"))
        .map(A::try_from)
        .collect::<Result<Vec<A>>>()?
        .into_iter())
}

/// Parse all `#[serde(..)]` attributes from the given slice.
// #[cfg(feature = "serde-compat")]
#[allow(unused)]
pub fn parse_serde_attrs<'a, A: TryFrom<&'a Attribute, Error = syn::Error>>(
    attrs: &'a [Attribute],
) -> impl Iterator<Item = A> {
    attrs
        .iter()
        .filter(|a| a.path.is_ident("serde"))
        .flat_map(|attr| match A::try_from(attr) {
            Ok(attr) => Some(attr),
            Err(_) => {
                use quote::ToTokens;
                // warning::print_warning(
                //     "failed to parse serde attribute",
                //     format!("{}", attr.to_token_stream()),
                //     "ts-rs failed to parse this attribute. It will be ignored.",
                // )
                // .unwrap();
                None
            }
        })
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
