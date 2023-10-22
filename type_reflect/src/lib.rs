#![feature(specialization)]
extern crate type_reflect_macros;
pub use core::convert::AsRef;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
pub use std::io::Write;
pub use std::path::Path;

pub use type_reflect_macros::export_types;
pub use type_reflect_macros::Reflect;
pub mod struct_type;
pub use struct_type::*;
pub use type_reflect_core::*;
pub mod zod;
pub use zod::Zod;
pub mod rust;
pub use rust::Rust;
pub mod enum_type;
pub use enum_type::*;
pub mod alias_type;
pub use alias_type::*;

pub use serde::{Deserialize, Serialize};
pub use serde_json;

// pub mod macros {
//     pub use my_proc_macro::MyProcMacro;
// }

pub trait Emittable {
    fn emit_with<E: TypeEmitter>() -> String;
}

pub trait TypeEmitter {
    fn init_destination_file<P: std::fmt::Debug + Clone>(path: P) -> Result<File, std::io::Error>
    where
        P: AsRef<Path>,
    {
        let mut file = match File::create(path.clone()) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Error creating file: {:?}", path);
                return Err(err);
            }
        };
        file.write_all(Self::dependencies().as_bytes())?;
        Ok(file)
    }

    fn finalize<P>(path: P) -> Result<(), std::io::Error>
    where
        P: AsRef<OsStr>;

    fn dependencies() -> String;
    fn emit<T: Emittable>() -> String
    where
        Self: Sized,
    {
        T::emit_with::<Self>()
    }
    // }

    // pub trait StructTypeEmitter {
    fn emit_struct<T>() -> String
    where
        T: StructType;
    // }

    // pub trait EnumTypeEmitter {
    fn emit_enum<T>() -> String
    where
        T: EnumReflectionType;

    fn emit_alias<T>() -> String
    where
        T: AliasType;
}

pub trait RustType {
    fn emit_rust(&self) -> String;
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
