use crate::zod::to_zod_type;
use ts_quote::*;
use type_reflect_core::{Inflection, NamedField, TypeFieldsDefinition};

pub fn struct_member(member: &NamedField, inflection: Inflection) -> String {
    let name = crate::ts_key(&member.serialized_name(inflection));
    let value = to_zod_type(&member.type_);
    ts_string! { #name: #value, }

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
        // Unit structs are rejected at derive time; tuple structs are
        // handled directly by `emit_struct` (they must not be wrapped in
        // `z.object`).
        TypeFieldsDefinition::Unit => panic!("unit structs are not supported"),
        TypeFieldsDefinition::Tuple(_) => {
            panic!("tuple structs are handled by emit_struct, not struct_fields")
        }
        TypeFieldsDefinition::Named(named) => named_fields(named, inflection),
    }
}
