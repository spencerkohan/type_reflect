use ts_quote::ts_string;
use type_reflect_core::EnumCase;

use crate::ts_validation::{
    struct_type::named_field_validations, validation::tuple_validation, validation_namespace,
};

pub fn emit_case_type(case: &EnumCase, parent_name: &str) -> String {
    let case_type = format!("{}Case{}", parent_name, case.name);
    let validation_impl = match &case.type_ {
        type_reflect_core::TypeFieldsDefinition::Unit => {
            unreachable!("Unit cases don't emit case types");
        }
        type_reflect_core::TypeFieldsDefinition::Tuple(members) => {
            if members.len() == 1 {
                return "".to_string();
            }
            tuple_validation("input", &members)
        }
        type_reflect_core::TypeFieldsDefinition::Named(fields) => {
            let val = named_field_validations("input", &fields, case.inflection);
            ts_string! {
                if (!isRecord(input)) {
                    throw new Error(# "`Error parsing #case_type: expected: Record, found: ${typeof input}`");
                }
                #val
            }
        }
    };

    let validation_impl = ts_string! {
        #validation_impl
        return input as #case_type;
    };

    validation_namespace(&case_type, &validation_impl)
}
