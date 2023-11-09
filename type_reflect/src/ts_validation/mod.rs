use std::ffi::OsStr;

use crate::{AliasType, EnumReflectionType, StructType, TypeEmitter};

mod struct_type;
use struct_type::struct_impl;

mod validation;

#[derive(Default)]
pub struct TSValidation {}

impl TypeEmitter for TSValidation {
    fn prefix(&mut self) -> String {
        r#"
            export type Result<T> = {ok: true, value: T}
                | {ok: false, error: Error };

            function isRecord(value: any): value is Record<string, any> {
                return typeof value === 'object' && value !== null && !Array.isArray(value);
            }
        "#
        .to_string()
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
        "".to_string()
    }

    fn emit_alias<T>(&mut self) -> String
    where
        T: AliasType,
    {
        "".to_string()
    }

    fn finalize<P>(&mut self, _path: P) -> Result<(), std::io::Error>
    where
        P: AsRef<OsStr>,
    {
        Ok(())
    }
}
