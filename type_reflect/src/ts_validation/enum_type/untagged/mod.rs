use case_type::emit_case_type;
use ts_quote::ts_string;
use type_reflect_core::{EnumCase, TypeFieldsDefinition};
use union_case::union_case_validation;
use unit_case::unit_case_validation;

use crate::{EnumReflectionType, ts_validation::validation_namespace};
mod case_type;
mod union_case;
mod unit_case;

pub fn emit_untagged_enum_type<T>() -> String
where
    T: EnumReflectionType,
{
    let name = T::name();
    let cases = T::cases();
    let inflection = T::inflection();

    let unit_cases: Vec<String> = cases
        .iter()
        .filter(|c| {
            if let TypeFieldsDefinition::Unit = c.type_ {
                true
            } else {
                false
            }
        })
        .map(|case| case.serialized_name(inflection))
        .collect();

    let unit_case_validations = if unit_cases.is_empty() {
        "".to_string()
    } else {
        let unit_case_validations: Vec<_> = unit_cases
            .into_iter()
            .map(|case_name| unit_case_validation(case_name.as_str(), &name))
            .collect();
        let unit_case_validations = unit_case_validations.join("\n");
        ts_string! {
            if (# "'string'" === typeof input) {
                #unit_case_validations
                throw new Error(# "`Error validating #name: none of the unit cases were matched`");
            }
        }
    };

    let union_cases: Vec<&EnumCase> = cases
        .iter()
        .filter(|c| {
            if let TypeFieldsDefinition::Unit = c.type_ {
                false
            } else {
                true
            }
        })
        .collect();

    let union_case_validations = if union_cases.is_empty() {
        "".to_string()
    } else {
        let union_case_validations: Vec<_> = union_cases
            .iter()
            .map(|case| union_case_validation(case, &name, inflection))
            .collect();
        let union_case_validations = union_case_validations.join("\n");
        ts_string! {
            if (!isRecord(input)) {
                throw new Error(# "`Error parsing #name: expected: Record, found: ${typeof input}`");
            }
            #union_case_validations
        }
    };

    let union_case_types: Vec<_> = union_cases
        .iter()
        .map(|case| emit_case_type(case, &name))
        .collect();

    let union_case_types = union_case_types.join("\n");

    let namespace = validation_namespace(
        &name,
        &ts_string! {
            #unit_case_validations
            #union_case_validations
            throw new Error(# "`Error validating #name: none of the union cases were matched`");
        },
    );

    ts_string! {

        #union_case_types
        #namespace
    }
}
