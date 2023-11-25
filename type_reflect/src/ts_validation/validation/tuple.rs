use crate::ts_validation::validation::type_validation;
use ts_quote::ts_string;
use type_reflect_core::Type;

pub fn tuple_validation(var_name: &str, member_types: &Vec<Type>) -> String {
    if member_types.len() == 1 {
        return type_validation(var_name, &member_types[0]);
    }

    let member_validations: String = member_types
        .into_iter()
        .enumerate()
        .map(|(i, member)| {
            type_validation(
                ts_string! {
                    #var_name[#i]
                }
                .as_str(),
                &member,
            )
        })
        .collect();

    ts_string! {
        if (!Array.isArray(#var_name)) {
            throw new Error(#"`Error parsing #var_name: expected: Array, found: ${ typeof #var_name }`");
        }
        #member_validations
    }
}
