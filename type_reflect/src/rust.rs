pub use super::struct_type::*;
pub use super::type_description::Type;
pub use super::*;
use std::ffi::OsStr;
use std::process::Command;

pub struct Rust {}

const DERIVES: &str = "#[derive(Debug, Clone, Serialize, Deserialize)]";

impl TypeEmitter for Rust {
    fn dependencies() -> String {
        "use serde::{Deserialize, Serialize};\nuse serde_json;\n".to_string()
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

    fn emit_alias<T>() -> String
    where
        T: AliasType,
    {
        format!("\n{}\n{}\n", DERIVES, T::rust())
    }

    fn finalize<P>(path: P) -> Result<(), std::io::Error>
    where
        P: AsRef<OsStr>,
    {
        let output = Command::new("rustfmt").arg(path).output()?;

        if !output.status.success() {
            eprintln!("Failed to format file");
            eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }

        Ok(())
    }
}
