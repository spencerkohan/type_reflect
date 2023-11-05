pub use super::struct_type::*;
pub use super::type_description::Type;
pub use super::*;
use std::ffi::OsStr;
use std::process::Command;

#[derive(Default)]
pub struct Rust {}

const DERIVES: &str = "#[derive(Debug, Clone, Serialize, Deserialize)]";

impl TypeEmitter for Rust {
    fn prefix(&mut self) -> String {
        "use serde::{Deserialize, Serialize};\nuse serde_json;\n".to_string()
    }

    fn emit_struct<T>(&mut self) -> String
    where
        T: StructType,
    {
        format!("\n{}\n{}\n", DERIVES, T::rust())
    }

    fn emit_enum<T>(&mut self) -> String
    where
        T: EnumReflectionType,
    {
        format!("\n{}\n{}\n", DERIVES, T::rust())
    }

    fn emit_alias<T>(&mut self) -> String
    where
        T: AliasType,
    {
        format!("\n{}\n{}\n", DERIVES, T::rust())
    }

    fn finalize<P>(&mut self, path: P) -> Result<(), std::io::Error>
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
