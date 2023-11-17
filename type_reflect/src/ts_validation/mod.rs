use std::ffi::OsStr;

use crate::{AliasType, EnumReflectionType, StructType, TypeEmitter};

mod struct_type;
use struct_type::struct_impl;

mod enum_type;
use enum_type::emit_enum_type;

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
        emit_enum_type::<T>()
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

pub fn validation_namespace(name: &str, validation_impl: &str) -> String {
    format!(
        r#"

export namespace {name} {{
    export function tryValidate(input: any): {name} {{
        {validation_impl}
    }}

    export function tryParse(input: string): {name} {{
        let json = JSON.parse(input);
        return tryValidate(json);
    }}

    export function validate(input: any): Result<{name}> {{
        try {{
            return {{ok: true, value: tryValidate(input)}};
        }} catch (e: any) {{
            return {{ok: false, error: e as Error}};
        }}
    }}

    export function parse(input: string): Result<{name}> {{
        let json = JSON.parse(input);
        return validate(json);
    }}

}}
        "#,
        name = name,
        validation_impl = validation_impl
    )
}
