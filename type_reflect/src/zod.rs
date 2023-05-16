pub use super::struct_type::*;
pub use super::type_description::Type;
pub use super::*;

pub struct Zod {}

pub trait TypeExporter {
    fn export<T>() -> String;
}

fn to_zod_type(t: &Type) -> String {
    match t {
        Type::Named(t) => format!("{}Schema", t),
        Type::String => "z.string()".to_string(),
        Type::Int => "z.number()".to_string(),
        Type::UnsignedInt => "z.number()".to_string(),
        Type::Float => "z.number()".to_string(),
        Type::Boolean => "z.bool()".to_string(),
        _ => todo!(),
    }
}

fn struct_member(member: &StructMember) -> String {
    let name = &member.name;
    let value = to_zod_type(&member.type_);
    format!("{name}: {value},", name = name, value = value)
}

fn struct_members(members: &Vec<StructMember>) -> String {
    let mut result = String::new();
    for member in members {
        result.push_str(struct_member(member).as_str())
    }
    result
}

impl TypeEmitter for Zod {
    fn emit<T>() -> String
    where
        T: StructType,
    {
        let members = struct_members(&T::members());
        let name = T::name();

        format!(
            r#"

export const {name}Schema = z.object({{
    {members}
}});

export type {name} = z.infer<typeof {name}Schema>;

"#,
            members = members,
            name = name
        )
    }

    fn dependencies() -> String {
        "import { z } from 'zod';".to_string()
    }
}
