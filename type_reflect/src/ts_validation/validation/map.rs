use type_reflect_core::Type;

use crate::ts_validation::validation::type_validation;

pub fn map_validation(var_name: &str, member_type: &Type) -> String {
    let validation = type_validation("item", member_type);
    format!(
        r#"
    if (!isRecord({var_name})) {{
        throw new Error(`Error parsing {var_name}: expected: Record, found: ${{ typeof {var_name} }}`);
    }}
    for (const key in {var_name}) {{
        const item = {var_name}[key];
        {validation}
    }}
    "#,
        var_name = var_name,
        validation = validation,
    )
}
