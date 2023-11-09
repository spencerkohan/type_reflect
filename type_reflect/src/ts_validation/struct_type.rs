use crate::type_script::to_ts_type;

use type_reflect_core::{Inflectable, Inflection, StructMember};

use super::validation::type_validation;

pub fn struct_member_validations(members: &Vec<StructMember>, inflection: Inflection) -> String {
    let members: Vec<String> = members
        .into_iter()
        .map(|member| {
            let member_name = member.name.inflect(inflection);
            type_validation(format!("input.{}", member_name).as_str(), &member.type_)
        })
        .collect();
    members.join("\n  ")
}

pub fn struct_impl(name: &str, members: &Vec<StructMember>, inflection: Inflection) -> String {
    let validaitions = struct_member_validations(members, inflection);

    return format!(
        r#"

export namespace {name} {{
    export function tryValidate(input: any): {name} {{
        if (!isRecord(input)) {{
            throw new Error(`Error parsing {name}: expected: Record, found: ${{typeof input}}`);
        }}
        {validaitions}
        return input as {name};
    }}

    export function tryParse(input: string): {name} {{
        let json = JSON.parse(input);
        return tryValidate(json);
    }}

    export function validate(input: any): Result<{name}> {{
        try {{
            return {{ok: true, value: tryValidate(input)}};
        }} catch (e: any) {{
            return {{ok: false, error: e as Error}};
        }}
    }}

    export function parse(input: string): Result<{name}> {{
        let json = JSON.parse(input);
        return validate(json);
    }}

}}
        "#,
        name = name,
        validaitions = validaitions
    );
}
