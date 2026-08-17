use type_reflect_core::{Inflection, NamedField, TypeFieldsDefinition};

use super::{
    validation::{tuple_validation, type_validation},
    validation_namespace,
};
use ts_quote::*;

pub fn named_field_validations(
    member_prefix: &str,
    members: &Vec<NamedField>,
    inflection: Inflection,
) -> String {
    let members: Vec<String> = members
        .into_iter()
        .map(|member| {
            let member_name = member.serialized_name(inflection);
            let member_var = crate::ts_access(member_prefix, &member_name);
            type_validation(&member_var, &member.type_)
        })
        .collect();
    members.join("\n  ")
}

pub fn struct_field_validations(
    member_prefix: &str,
    fields: &TypeFieldsDefinition,
    inflection: Inflection,
) -> String {
    match fields {
        // Unit structs are rejected at derive time.
        TypeFieldsDefinition::Unit => panic!("unit structs are not supported"),
        TypeFieldsDefinition::Tuple(tuple) => tuple_validation(member_prefix, tuple),
        TypeFieldsDefinition::Named(named) => {
            named_field_validations(member_prefix, named, inflection)
        }
    }
}

pub fn struct_impl(name: &str, fields: &TypeFieldsDefinition, inflection: Inflection) -> String {
    let validations = struct_field_validations("input", fields, inflection);

    let validation_impl = match fields {
        // Unit structs are rejected at derive time.
        TypeFieldsDefinition::Unit => panic!("unit structs are not supported"),
        TypeFieldsDefinition::Tuple(_) => {
            ts_string! {
                #validations
                return input as #name;
            }
        }
        TypeFieldsDefinition::Named(_) => ts_string! {
            if (!isRecord(input)) {
                throw new Error(# "`Error parsing #name: expected: Record, found: ${typeof input}`");
            }
            #validations
            return input as #name;
        },
    };

    //     let validation_impl = format!(
    //         r#"
    //         if (!isRecord(input)) {{
    //             throw new Error(`Error parsing {name}: expected: Record, found: ${{typeof input}}`);
    //         }}
    //         {validations}
    //         return input as {name};
    // "#,
    //         name = name,
    //         validations = validations
    //     );

    validation_namespace(name, validation_impl.as_str())
}
