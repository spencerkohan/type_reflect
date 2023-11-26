use ts_quote::ts_string;
use type_reflect_core::Type;

use crate::ts_validation::validation::type_validation;

pub fn array_validation(var_name: &str, member_type: &Type) -> String {
    let validation = type_validation("item", member_type);
    ts_string! {
        if (!Array.isArray(#var_name)) {
            throw new Error(#"`Error parsing #var_name: expected: Array, found: ${ typeof #var_name }`");
        }
        for (const item of #var_name) {
            #validation
        }
    }
}
