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

pub struct TypeScript {}

pub trait TypeExporter {
    fn export<T>() -> String;
}

fn to_ts_type(t: &Type) -> String {
    match t {
        Type::Named(t) => format!("{}", t),
        Type::String => "string".to_string(),
        Type::Int => "number".to_string(),
        Type::UnsignedInt => "number".to_string(),
        Type::Float => "number".to_string(),
        Type::Boolean => "boolean".to_string(),
        Type::Option(t) => format!("{}", to_ts_type(t)),
        Type::Array(t) => format!("Array<{}>", to_ts_type(t)),
        Type::Map { key, value } => format!("Map<{}, {}>", to_ts_type(key), to_ts_type(value)),
    }
}

impl TypeEmitter for TypeScript {
    fn dependencies() -> String {
        "".to_string()
    }

    fn emit_struct<T>() -> String
    where
        T: StructType,
    {
        let members = struct_members(&T::members(), T::inflection());
        let name = T::name();

        format!(
            r#"

export type {name} = {{
  {members}
}};

"#,
            members = members,
            name = name
        )
    }

    fn emit_enum<T>() -> String
    where
        T: EnumReflectionType,
    {
        emit_enum_type::<T>()
    }

    fn emit_alias<T>() -> String
    where
        T: AliasType,
    {
        emit_alias_type::<T>()
    }

    fn finalize<P>(_path: P) -> Result<(), std::io::Error>
    where
        P: AsRef<OsStr>,
    {
        Ok(())
    }
}
