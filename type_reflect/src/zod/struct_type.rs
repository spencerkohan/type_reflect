use crate::zod::to_zod_type;
use type_reflect_core::StructMember;

pub fn struct_member(member: &StructMember) -> String {
    let name = &member.name;
    let value = to_zod_type(&member.type_);
    format!("    {name}: {value},\n", name = name, value = value)
}

pub fn struct_members(members: &Vec<StructMember>) -> String {
    let mut result = String::new();
    for member in members {
        result.push_str(struct_member(member).as_str())
    }
    result
}
