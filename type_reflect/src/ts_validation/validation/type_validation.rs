use type_reflect_core::Type;

use super::{array_validation, map::map_validation, primitive_type_validation};

pub fn type_validation(var_name: &str, type_: &Type) -> String {
    match type_ {
        Type::String => primitive_type_validation(var_name, "string"),
        Type::Float | Type::Int | Type::UnsignedInt => {
            primitive_type_validation(var_name, "number")
        }
        Type::Boolean => primitive_type_validation(var_name, "boolean"),
        Type::Array(t) => array_validation(var_name, &t),
        Type::Map { key: _, value } => map_validation(var_name, value),
        Type::Option(t) => {
            let type_validation = type_validation(var_name, &t);
            format!(
                r#"
                if ({var_name}) {{
                    {type_validation}
                }}
                "#,
                var_name = var_name,
                type_validation = type_validation
            )
        }
        Type::Named(n) => {
            // let value_type = to_ts_type(t);
            let value_type = n;
            format!(
                r#"
                {value_type}.validate({var_name});
                "#,
                var_name = var_name,
                value_type = value_type
            )
        }
    }
}
