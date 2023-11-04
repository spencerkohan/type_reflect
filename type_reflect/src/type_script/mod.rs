use std::ffi::OsStr;

pub use super::struct_type::*;
pub use super::type_description::Type;
use super::*;

pub mod struct_type;
use struct_type::*;

pub mod enum_type;
pub use enum_type::*;

mod alias_type;
pub use alias_type::*;

pub struct TypeScript {
    pub tab_size: u32,
}

impl Default for TypeScript {
    fn default() -> Self {
        Self { tab_size: 2 }
    }
}

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
    fn dependencies(&mut self) -> String {
        "".to_string()
    }

    fn emit_struct<T>(&mut self) -> String
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
