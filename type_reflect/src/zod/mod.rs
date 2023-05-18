pub use super::struct_type::*;
pub use super::type_description::Type;
pub use super::*;

mod struct_type;
use struct_type::*;

mod enum_type;
use enum_type::*;

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
        Type::Option(t) => format!("{}.optional()", to_zod_type(t)),
        Type::Array(t) => format!("z.array({})", to_zod_type(t)),
        Type::Map { key, value } => format!("z.map({}, {})", to_zod_type(key), to_zod_type(value)),
        _ => todo!(),
    }
}

impl TypeEmitter for Zod {
    fn dependencies() -> String {
        "import { z } from 'zod';".to_string()
    }
    // }

    // impl StructTypeEmitter for Zod {
    fn emit_struct<T>() -> String
    where
        T: StructType,
    {
        let members = struct_members(&T::members(), T::inflection());
        let name = T::name();

        format!(
            r#"

export const {name}Schema = z.object({{
{members}}});

export type {name} = z.infer<typeof {name}Schema>;

"#,
            members = members,
            name = name
        )
    }
    // }

    // impl EnumTypeEmitter for Zod {
    fn emit_enum<T>() -> String
    where
        T: EnumReflectionType,
    {
        emit_enum_type::<T>()
    }
}
