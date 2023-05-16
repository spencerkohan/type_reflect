extern crate type_reflect_macros;
use std::fs;
use std::fs::File;
use std::io::Write;

pub use type_reflect_macros::export_types;
pub use type_reflect_macros::Reflect;
pub mod struct_type;
pub use struct_type::*;
pub use type_reflect_core::*;
pub mod zod;
pub use zod::Zod;

// pub mod macros {
//     pub use my_proc_macro::MyProcMacro;
// }

pub trait TypeEmitter {
    fn init_destination_file(path: &str) -> Result<File, std::io::Error> {
        let mut file = File::create(path)?;
        file.write_all(Self::dependencies().as_bytes());
        Ok(file)
    }

    fn emit_into<T>(file: &mut File) -> Result<(), std::io::Error>
    where
        T: StructType,
    {
        file.write_all(Self::emit::<T>().as_bytes())
    }

    fn dependencies() -> String;
    fn emit<T>() -> String
    where
        T: StructType;
}

pub trait RustType {
    fn emit_rust(&self) -> String;
}

pub trait EnumType {}

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
