use std::ffi::OsStr;

pub use super::struct_type::*;
pub use super::type_description::Type;
use super::*;

pub mod struct_type;
use dprint_plugin_typescript::{configuration::ConfigurationBuilder, FormatTextOptions};
use struct_type::*;

pub mod enum_type;
pub use enum_type::*;
pub mod externally_tagged_enum_type;
pub mod untagged_enum_type;

pub mod type_fields;
pub use type_fields::*;

mod alias_type;
pub use alias_type::*;

#[cfg(test)]
mod tests {
    use super::*;
    use type_reflect_core::Type;

    #[test]
    fn test_to_ts_type_built_ins() {
        assert_eq!(to_ts_type(&Type::String), "string");
        assert_eq!(to_ts_type(&Type::JsonValue), "any");
        assert_eq!(to_ts_type(&Type::Option(Box::new(Type::JsonValue))), "any");
        assert_eq!(
            to_ts_type(&Type::Array(Box::new(Type::JsonValue))),
            "Array<any>"
        );
    }
}

pub struct TypeScript {
    pub tab_size: u32,
}

impl Default for TypeScript {
    fn default() -> Self {
        Self { tab_size: 2 }
    }
}

pub fn to_ts_type(t: &Type) -> String {
    match t {
        // TODO: Support generics
        Type::Named(t) => format!("{}", t.name),
        Type::String => "string".to_string(),
        Type::Int => "number".to_string(),
        Type::UnsignedInt => "number".to_string(),
        Type::Float => "number".to_string(),
        Type::Boolean => "boolean".to_string(),
        Type::JsonValue => "any".to_string(),
        Type::Option(t) => format!("{}", to_ts_type(t)),
        Type::Array(t) => format!("Array<{}>", to_ts_type(t)),
        Type::Map { key, value } => {
            format!(
                "{{[key: {k}]: {v}}}",
                k = to_ts_type(key),
                v = to_ts_type(value)
            )
        }
        Type::Transparent(t) => to_ts_type(&*(t.type_)),
    }
}

impl TypeEmitter for TypeScript {
    fn prefix(&mut self) -> String {
        "".to_string()
    }

    fn emit_struct<T>(&mut self) -> String
    where
        T: StructType,
    {
        let name = T::name();
        struct_impl(&name, &T::fields(), T::inflection())
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

    fn finalize<P>(&mut self, path: P) -> Result<(), std::io::Error>
    where
        P: AsRef<OsStr>,
    {
        // build the configuration once
        let config = ConfigurationBuilder::new()
            .indent_width(self.tab_size as u8)
            .line_width(80)
            .build();

        let file_path = Path::new(&path);

        let text: String = std::fs::read_to_string(Path::new(&path))?;

        let options: FormatTextOptions = FormatTextOptions {
            path: Path::new(&path),
            extension: None,
            text,
            config: &config,
            external_formatter: None,
        };

        let result = dprint_plugin_typescript::format_text(options);

        match result {
            Ok(Some(contents)) => {
                std::fs::write(file_path, contents)?;
            }
            Err(e) => {
                eprintln!("Error formatting typescript: {}", e);
            }
            _ => {}
        };

        Ok(())
    }
}
