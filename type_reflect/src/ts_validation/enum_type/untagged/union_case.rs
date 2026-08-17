use ts_quote::ts_string;
use type_reflect_core::{EnumCase, Inflectable, Inflection, Type};

use crate::{
    ts_validation::validation::type_validation,
    type_script::untagged_enum_type::emit_case_type_name,
};

pub fn union_case_validation(case: &EnumCase, parent_name: &str, inflection: Inflection) -> String {
    let case_key = case.serialized_name(inflection);
    let case_type_name = emit_case_type_name(case, parent_name);

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
        if (input.#case_key) {
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
    if tuple_members.len() == 1 {
        let Some(case_type) = tuple_members.first() else {
            return "_ERROR_NO_CASE_TYPE_EXISTS_".to_string();
        };
        let var_name = ts_string! { input.#case_key };
        let val = type_validation(&var_name, case_type);
        ts_string! {
            #val
            return input as #parent_name;
        }
    } else {
        let case_type = format!("{}Case{}", parent_name, case.name);
        ts_string! {
            return { #case_key: #case_type.validate(input.#case_key) };
        }
    }
}

fn validate_struct_case(case: &EnumCase, parent_name: &str, case_key: &str) -> String {
    let case_type = format!("{}Case{}", parent_name, case.name);
    ts_string! {
        return { #case_key: #case_type.validate(input.#case_key) };
    }
}
