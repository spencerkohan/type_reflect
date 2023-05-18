use std::fmt::format;

pub use super::struct_type::*;
pub use super::type_description::Type;
pub use super::*;

pub struct Rust {}

const DERIVES: &str = "#[derive(Debug, Clone, Serialize, Deserialize)]";

impl TypeEmitter for Rust {
    fn dependencies() -> String {
        "use type_reflect::*;\n".to_string()
    }
    // }

    // impl StructTypeEmitter for Rust {
    fn emit_struct<T>() -> String
    where
        T: StructType,
    {
        format!("\n{}\n{}\n", DERIVES, T::rust())
    }
    // }

    // impl EnumTypeEmitter for Rust {
    fn emit_enum<T>() -> String
    where
        T: EnumReflectionType,
    {
        format!("\n{}\n{}\n", DERIVES, T::rust())
    }
}
