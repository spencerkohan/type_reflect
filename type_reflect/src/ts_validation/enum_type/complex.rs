use type_reflect_core::EnumCase;

use crate::{ts_validation::validation_namespace, EnumReflectionType};

use super::case_type::emit_complex_enum_case_type;
use ts_quote::ts_str;

pub fn emit_complex_enum_type<T>(case_key: &String, content_key: &Option<String>) -> String
where
    T: EnumReflectionType,
{
    let case_type_validators: String = T::cases()
        .into_iter()
        .map(|case: EnumCase| emit_complex_enum_case_type(T::name(), case_key, content_key, case))
        .collect();

    let case_validations: String = T::cases()
        .into_iter()
        .map(|case: EnumCase| validate_case(T::name(), &case))
        .collect();

    let name = T::name();

    let namespace = validation_namespace(T::name(), ts_str! {
        #case_validations
        throw new Error(#"`Error validating #name: value ${JSON.stringify(input)} does not match any variant`");
    }.as_str());

    ts_str! {
        #case_type_validators
        #namespace
    }
}

fn validate_case(type_name: &str, case: &EnumCase) -> String {
    let case_type = format!("{}Case{}", type_name, case.name);

    ts_str! {
        try {
            return #case_type.tryValidate(input);
        } catch {}
    }
}
