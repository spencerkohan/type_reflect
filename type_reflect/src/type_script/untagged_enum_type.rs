use ts_quote::ts_string;
use type_reflect_core::TypeFieldsDefinition;

use crate::{type_script::externally_tagged_enum_type::*, EnumReflectionType};

/// Serde's `#[serde(untagged)]` representation: each case serializes to its
/// bare content (struct -> `{ fields }`, tuple -> `[a, b]`, unit -> `null`),
/// with no case name anywhere in the output.
pub fn emit_untagged_enum_type<T>() -> String
where
    T: EnumReflectionType,
{
    let name = T::name();
    let cases = T::cases();

    let members: Vec<String> = cases
        .iter()
        .map(|case| {
            if let TypeFieldsDefinition::Unit = case.type_ {
                "null".to_string()
            } else {
                emit_case_type_name(case, &name)
            }
        })
        .collect();

    // Helper type declarations for the non-inline case contents.
    let case_types: Vec<String> = cases
        .iter()
        .filter(|case| !matches!(case.type_, TypeFieldsDefinition::Unit))
        .map(|case| emit_case_type(case, &name))
        .filter(|decl| !decl.is_empty())
        .collect();
    let case_types = case_types.join("\n");

    let members = members.join(" | ");

    ts_string! {
        #case_types

        export type #name = #members;
    }
}
