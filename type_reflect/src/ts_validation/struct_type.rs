use type_reflect_core::{Inflectable, Inflection, StructMember};

use super::{validation::type_validation, validation_namespace};

pub fn struct_member_validations(
    member_prefix: &str,
    members: &Vec<StructMember>,
    inflection: Inflection,
) -> String {
    let members: Vec<String> = members
        .into_iter()
        .map(|member| {
            let member_name = member.name.inflect(inflection);
            type_validation(
                format!("{}.{}", member_prefix, member_name).as_str(),
                &member.type_,
            )
        })
        .collect();
    members.join("\n  ")
}

pub fn struct_impl(name: &str, members: &Vec<StructMember>, inflection: Inflection) -> String {
    let validations = struct_member_validations("input", members, inflection);

    let validation_impl = format!(
        r#"
        if (!isRecord(input)) {{
            throw new Error(`Error parsing {name}: expected: Record, found: ${{typeof input}}`);
        }}
        {validations}
        return input as {name};
"#,
        name = name,
        validations = validations
    );

    validation_namespace(name, validation_impl.as_str())

    //     return format!(
    //         r#"

    // export namespace {name} {{
    //     export function tryValidate(input: any): {name} {{
    //         if (!isRecord(input)) {{
    //             throw new Error(`Error parsing {name}: expected: Record, found: ${{typeof input}}`);
    //         }}
    //         {validaitions}
    //         return input as {name};
    //     }}

    //     export function tryParse(input: string): {name} {{
    //         let json = JSON.parse(input);
    //         return tryValidate(json);
    //     }}

    //     export function validate(input: any): Result<{name}> {{
    //         try {{
    //             return {{ok: true, value: tryValidate(input)}};
    //         }} catch (e: any) {{
    //             return {{ok: false, error: e as Error}};
    //         }}
    //     }}

    //     export function parse(input: string): Result<{name}> {{
    //         let json = JSON.parse(input);
    //         return validate(json);
    //     }}

    // }}
    //         "#,
    //         name = name,
    //         validaitions = validaitions
    //     );
}
