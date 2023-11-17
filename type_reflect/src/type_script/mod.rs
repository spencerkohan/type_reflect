use std::ffi::OsStr;

pub use super::struct_type::*;
pub use super::type_description::Type;
use super::*;

pub mod struct_type;
use dprint_plugin_typescript::configuration::{
    ConfigurationBuilder, NextControlFlowPosition, QuoteStyle,
};
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

pub fn to_ts_type(t: &Type) -> String {
    match t {
        Type::Named(t) => format!("{}", t),
        Type::String => "string".to_string(),
        Type::Int => "number".to_string(),
        Type::UnsignedInt => "number".to_string(),
        Type::Float => "number".to_string(),
        Type::Boolean => "boolean".to_string(),
        Type::Option(t) => format!("{}", to_ts_type(t)),
        Type::Array(t) => format!("Array<{}>", to_ts_type(t)),
        Type::Map { key, value } => {
            format!(
                "{{[key: {k}]: {v}}}",
                k = to_ts_type(key),
                v = to_ts_type(value)
            )
        }
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
        struct_impl(&name, &T::members(), T::inflection())
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
            .prefer_hanging(true)
            .prefer_single_line(false)
            .quote_style(QuoteStyle::PreferSingle)
            .next_control_flow_position(NextControlFlowPosition::SameLine)
            .build();

        let file_path = Path::new(&path);

        let text: String = std::fs::read_to_string(Path::new(&path))?;

        let result =
            dprint_plugin_typescript::format_text(Path::new(&path), text.as_str(), &config);

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
