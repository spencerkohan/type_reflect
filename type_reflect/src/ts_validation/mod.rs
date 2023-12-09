use std::ffi::OsStr;

use crate::{AliasType, EnumReflectionType, StructType, TypeEmitter};

mod struct_type;
use struct_type::struct_impl;

mod enum_type;
use enum_type::emit_enum_type;
use ts_quote::ts_string;

mod validation;

#[derive(Default)]
pub struct TSValidation {}

impl TypeEmitter for TSValidation {
    fn prefix(&mut self) -> String {
        r#"
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
    ts_string! {
        export namespace #name {
            export function validate(input: any): #name {
                #validation_impl
            }

            export function parse(input: string): #name {
                let json = JSON.parse(input);
                return validate(json);
            }

            export function tryValidate(input: any): #name | undefined {
                try {
                    return validate(input);
                } catch {
                    return undefined;
                }
            }

            export function tryParse(input: string): #name | undefined {
                let json = JSON.parse(input);
                return tryValidate(json);
            }

            export function validateArray(input: any): Array<#name> {
                if (!Array.isArray(input)) {
                    throw new Error(#"`Error validating Array<#name>: expected: Array, found: ${ typeof input }`");
                }
                for (const item of input) {
                    validate(item);
                }
                return input as Array<#name>;
            }

            export function parseArray(input: string): Array<#name> {
                let json = JSON.parse(input);
                return validateArray(json);
            }

            export function tryValidateArray(input: any): Array<#name> | undefined {
                try {
                    return validateArray(input);
                } catch (e: any) {
                    return undefined;
                }
            }

            export function tryParseArray(input: any): Array<#name> | undefined {
                try {
                    return parseArray(input);
                } catch (e: any) {
                    return undefined;
                }
            }
        }
    }
}
