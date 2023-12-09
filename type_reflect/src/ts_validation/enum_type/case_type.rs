use ts_quote::ts_string;
use type_reflect_core::{EnumCase, Inflection, NamedField, Type};

use crate::ts_validation::{
    struct_type::named_field_validations, validation::tuple_validation, validation_namespace,
};

pub fn emit_complex_enum_case_type(
    enum_name: &str,
    case_key: &String,
    content_key: &Option<String>,
    case: EnumCase,
) -> String {
    let case_key_value: String = format!("{}Case.{}", enum_name, case.name);
    let case_type_name: String = format!("{}Case{}", enum_name, case.name);

    let validator = match case.type_ {
        type_reflect_core::TypeFieldsDefinition::Unit => emit_simple_case_type_validator(),
        type_reflect_core::TypeFieldsDefinition::Tuple(members) => {
            emit_tuple_case_type_validator(content_key, &members)
        }
        type_reflect_core::TypeFieldsDefinition::Named(members) => {
            emit_struct_case_type_validator(content_key, &members, case.inflection)
        }
    };

    let validation_impl = ts_string! {
        if (!isRecord(input)) {
            throw new Error(#"`Error parsing #case_type_name: expected: Record, found: ${typeof input}`");
        }
        if (input.#case_key !== #case_key_value) {
            throw new Error(#"`Error parsing #case_type_name: expected key: #case_key_value, found: ${typeof input}`");
        }
        #validator
        return input as #case_type_name
    };

    return validation_namespace(case_type_name.as_str(), validation_impl.as_str());
}

fn emit_simple_case_type_validator() -> String {
    String::new()
}

fn emit_struct_case_type_validator(
    content_key: &Option<String>,
    members: &Vec<NamedField>,
    inflection: Inflection,
) -> String {
    let member_prefix = match content_key {
        None => "input".to_string(),
        Some(key) => format!("input.{}", key),
    };
    named_field_validations(member_prefix.as_str(), members, inflection)
}

fn emit_tuple_case_type_validator(content_key: &Option<String>, members: &Vec<Type>) -> String {
    let member_prefix = match content_key {
        None => "input".to_string(),
        Some(key) => format!("input.{}", key),
    };
    tuple_validation(member_prefix.as_str(), members)
}
