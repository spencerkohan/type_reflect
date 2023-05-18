use crate::zod::to_zod_type;
use type_reflect_core::{Inflectable, Inflection, StructMember};

pub fn struct_member(member: &StructMember, inflection: Inflection) -> String {
    let name = &member.name.inflect(inflection);
    let value = to_zod_type(&member.type_);
    format!("    {name}: {value},\n", name = name, value = value)
}

pub fn struct_members(members: &Vec<StructMember>, inflection: Inflection) -> String {
    let mut result = String::new();
    for member in members {
        result.push_str(struct_member(member, inflection).as_str())
    }
    result
}
