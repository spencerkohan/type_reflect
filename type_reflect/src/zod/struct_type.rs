use crate::zod::to_zod_type;
use ts_quote::*;
use type_reflect_core::{Inflectable, Inflection, NamedField, TypeFieldsDefinition};

pub fn struct_member(member: &NamedField, inflection: Inflection) -> String {
    let name = &member.name.inflect(inflection);
    let value = to_zod_type(&member.type_);
    ts_string! { {name}: {value}, }

    // format!("    {name}: {value},\n", name = name, value = value)
}

pub fn named_fields(fields: &Vec<NamedField>, inflection: Inflection) -> String {
    let mut result = String::new();
    for member in fields {
        result.push_str(struct_member(member, inflection).as_str())
    }
    result
}

pub fn struct_fields(fields: &TypeFieldsDefinition, inflection: Inflection) -> String {
    match fields {
        TypeFieldsDefinition::Unit => todo!(),
        TypeFieldsDefinition::Tuple(_) => todo!(),
        TypeFieldsDefinition::Named(named) => named_fields(named, inflection),
    }
}
