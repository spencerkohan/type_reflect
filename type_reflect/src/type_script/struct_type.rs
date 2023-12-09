use super::to_ts_type;
use type_reflect_core::{Inflectable, Inflection, NamedField, TypeFieldsDefinition};

pub fn struct_member(member: &NamedField, inflection: Inflection) -> String {
    let name = &member.name.inflect(inflection);
    match &member.type_ {
        type_reflect_core::Type::Option(t) => {
            let value = to_ts_type(&t);
            format!("{name}?: {value};", name = name, value = value)
        }
        t => {
            let value = to_ts_type(&t);
            format!("{name}: {value};", name = name, value = value)
        }
    }
}

pub fn named_fields(members: &Vec<NamedField>, inflection: Inflection) -> String {
    let members: Vec<String> = members
        .into_iter()
        .map(|member| struct_member(member, inflection))
        .collect();
    members.join("\n  ")
}

pub fn struct_impl(name: &str, fields: &TypeFieldsDefinition, inflection: Inflection) -> String {
    let fields = match fields {
        TypeFieldsDefinition::Unit => todo!(),
        TypeFieldsDefinition::Tuple(_) => todo!(),
        TypeFieldsDefinition::Named(named) => named_fields(named, inflection),
    };
    return format!(
        r#"

export type {name} = {{
    {fields}
}};
        "#,
        name = name,
        fields = fields
    );
}
