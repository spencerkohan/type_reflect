use ts_quote::ts_string;
use type_reflect_core::{EnumCase, Inflection, Type};

use crate::ts_validation::validation::type_validation;

pub fn union_case_validation(case: &EnumCase, parent_name: &str, inflection: Inflection) -> String {
    let case_key = case.serialized_name(inflection);
    let case_key_access = crate::ts_access("input", &case_key);

    let case_validation = match &case.type_ {
        type_reflect_core::TypeFieldsDefinition::Unit => {
            unreachable!("Unit cases are handled separately");
        }
        type_reflect_core::TypeFieldsDefinition::Tuple(items) => {
            validate_tuple_case(case, &items, parent_name, &case_key)
        }
        type_reflect_core::TypeFieldsDefinition::Named(_) => {
            validate_struct_case(case, parent_name, &case_key)
        }
    };

    ts_string! {
        if (#case_key_access) {
            #case_validation
        }
    }
}

fn validate_tuple_case(
    case: &EnumCase,
    tuple_members: &Vec<Type>,
    parent_name: &str,
    case_key: &str,
) -> String {
    let case_key_access = crate::ts_access("input", case_key);
    let case_key_quoted = crate::ts_key(case_key);
    if tuple_members.len() == 1 {
        let Some(case_type) = tuple_members.first() else {
            return "_ERROR_NO_CASE_TYPE_EXISTS_".to_string();
        };
        let val = type_validation(&case_key_access, case_type);
        ts_string! {
            #val
            return input as #parent_name;
        }
    } else {
        let case_type = format!("{}Case{}", parent_name, case.name);
        ts_string! {
            return { #case_key_quoted: #case_type.validate(#case_key_access) };
        }
    }
}

fn validate_struct_case(case: &EnumCase, parent_name: &str, case_key: &str) -> String {
    let case_type = format!("{}Case{}", parent_name, case.name);
    let case_key_access = crate::ts_access("input", case_key);
    let case_key_quoted = crate::ts_key(case_key);
    ts_string! {
        return { #case_key_quoted: #case_type.validate(#case_key_access) };
    }
}
