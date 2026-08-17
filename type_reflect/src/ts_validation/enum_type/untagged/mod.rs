use ts_quote::ts_string;
use type_reflect_core::{EnumCase, TypeFieldsDefinition};

use crate::{
    ts_validation::{validation::type_validation, validation_namespace},
    EnumReflectionType,
};

use super::externally_tagged::emit_case_type;

/// Serde's `#[serde(untagged)]` representation: the variant content is
/// validated bare (no case key). Cases are tried in declaration order and
/// the first one to validate wins — the same order serde's untagged
/// deserialization uses.
pub fn emit_untagged_enum_type<T>() -> String
where
    T: EnumReflectionType,
{
    let name = T::name();
    let cases = T::cases();

    // Per-case content validator namespaces for the non-inline contents
    // (multi-element tuples and struct cases).
    let case_types: Vec<String> = cases
        .iter()
        .filter(|case| needs_case_type(case))
        .map(|case| emit_case_type(case, &name))
        .collect();
    let case_types = case_types.join("\n");

    let case_validations: Vec<String> = cases
        .iter()
        .map(|case| case_validation(case, &name))
        .collect();
    let case_validations = case_validations.join("\n");

    let namespace = validation_namespace(
        &name,
        &ts_string! {
            #case_validations
            throw new Error(# "`Error validating #name: none of the untagged cases matched`");
        },
    );

    ts_string! {

        #case_types
        #namespace
    }
}

fn needs_case_type(case: &EnumCase) -> bool {
    match &case.type_ {
        TypeFieldsDefinition::Named(_) => true,
        TypeFieldsDefinition::Tuple(items) => items.len() > 1,
        TypeFieldsDefinition::Unit => false,
    }
}

/// One case of the first-match-wins chain: try to validate the bare input as
/// this case's content; if validation throws, the next case is tried.
fn case_validation(case: &EnumCase, parent_name: &str) -> String {
    let body = match &case.type_ {
        TypeFieldsDefinition::Unit => ts_string! {
            if (input === null) {
                return input as #parent_name
            }
            throw new Error(# "`Error validating #parent_name: not a unit case`");
        },
        TypeFieldsDefinition::Tuple(items) if items.len() == 1 => {
            let Some(inner) = items.first() else {
                return String::new();
            };
            let val = type_validation("input", inner);
            ts_string! {
                #val
                return input as #parent_name
            }
        }
        _ => {
            let case_type = format!("{}Case{}", parent_name, case.name);
            ts_string! {
                return #case_type.validate(input);
            }
        }
    };

    ts_string! {
        try {
            #body
        } catch {}
    }
}
