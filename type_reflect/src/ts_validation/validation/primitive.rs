pub fn primitive_type_validation(var_name: &str, primitive_type: &str) -> String {
    format!(
        r#"
    if ('{primitive_type}' !== typeof {var_name}) {{
        throw new Error(`Validation error: expected: {primitive_type}, found: ${{ typeof {var_name} }}`);
    }}
    "#,
        var_name = var_name,
        primitive_type = primitive_type
    )
}

// Better error?
// throw new Error(`Error parsing {parent_name}.{name}: expected: {primitive}, found: ${{ typeof input.{name} }}`);
