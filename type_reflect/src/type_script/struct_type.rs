use super::to_ts_type;
use type_reflect_core::{Inflectable, Inflection, StructMember};

pub fn struct_member(member: &StructMember, inflection: Inflection) -> String {
    let name = &member.name.inflect(inflection);
    let value = to_ts_type(&member.type_);
    format!("{name}: {value};", name = name, value = value)
}

pub fn struct_members(members: &Vec<StructMember>, inflection: Inflection) -> String {
    let members: Vec<String> = members
        .into_iter()
        .map(|member| struct_member(member, inflection))
        .collect();
    members.join("\n  ")
}

pub fn struct_impl(name: &str, members: &Vec<StructMember>, inflection: Inflection) -> String {
    let members = struct_members(members, inflection);
    return format!(
        r#"

export type {name} = {{
    {members}
}};
        "#,
        name = name,
        members = members
    );
}
