use std::ffi::OsStr;

pub use super::struct_type::*;
pub use super::type_description::Type;
use super::*;

mod struct_type;
use struct_type::*;

mod enum_type;
use enum_type::*;

mod alias_type;
use alias_type::*;

#[derive(Default)]
pub struct Zod {}

pub trait TypeExporter {
    fn export<T>() -> String;
}

fn to_zod_type(t: &Type) -> String {
    match t {
        // TODO: support generics
        Type::Named(t) => format!("{}Schema", t.name),
        Type::String => "z.string()".to_string(),
        Type::Int => "z.number()".to_string(),
        Type::UnsignedInt => "z.number()".to_string(),
        Type::Float => "z.number()".to_string(),
        Type::Boolean => "z.bool()".to_string(),
        Type::Option(t) => format!("{}.optional()", to_zod_type(t)),
        Type::Array(t) => format!("z.array({})", to_zod_type(t)),
        Type::Map { key, value } => format!("z.map({}, {})", to_zod_type(key), to_zod_type(value)),
        Type::Transparent(_t) => unimplemented!("Transparent types not yet implemented for Zod"),
    }
}

impl TypeEmitter for Zod {
    fn prefix(&mut self) -> String {
        "import { z } from 'zod';\n".to_string()
    }

    fn emit_struct<T>(&mut self) -> String
    where
        T: StructType,
    {
        let name = T::name();

        // Tuple structs mirror serde: a single field serializes as the bare
        // value, multiple fields as an array — so no `z.object` wrapper.
        let schema = match &T::fields() {
            TypeFieldsDefinition::Tuple(tuple) if tuple.len() > 1 => {
                let items: Vec<String> = tuple.iter().map(|t| to_zod_type(t)).collect();
                format!("z.tuple([{}])", items.join(", "))
            }
            TypeFieldsDefinition::Tuple(tuple) => to_zod_type(&tuple[0]),
            fields => {
                let members = struct_fields(fields, T::inflection());
                format!(
                    r#"z.object({{
{members}}})"#
                )
            }
        };

        format!(
            r#"

export const {name}Schema = {schema};

export type {name} = z.infer<typeof {name}Schema>;

"#,
            schema = schema,
            name = name
        )
    }

    fn emit_enum<T>(&mut self) -> String
    where
        T: EnumReflectionType,
    {
        emit_enum_type::<T>()
    }

    fn emit_alias<T>(&mut self) -> String
    where
        T: AliasType,
    {
        emit_alias_type::<T>()
    }

    fn finalize<P>(&mut self, _path: P) -> Result<(), std::io::Error>
    where
        P: AsRef<OsStr>,
    {
        Ok(())
    }
}
