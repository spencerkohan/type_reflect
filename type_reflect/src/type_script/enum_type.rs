use ts_quote::ts_string;
use type_reflect_core::{EnumCase, EnumType, Inflection};

use crate::type_script::type_fields;
use crate::EnumReflectionType;

use super::to_ts_type;

pub fn emit_enum_type<T>() -> String
where
    T: EnumReflectionType,
{
    match T::enum_type() {
        EnumType::Simple => emit_simple_enum_type::<T>(),
        EnumType::Complex {
            case_key,
            content_key,
        } => emit_complex_enum_type::<T>(&case_key, &content_key),
    }
}

fn emit_simple_enum_type<T>() -> String
where
    T: EnumReflectionType,
{
    let simple_cases: String = T::cases()
        .into_iter()
        .map(|case| {
            format!(
                r#"  {name} = "{name}",
"#,
                name = case.name
            )
        })
        .collect();

    format!(
        r#"
export enum {name} {{
{simple_cases}}}
"#,
        name = T::name(),
        simple_cases = simple_cases,
    )
}

fn emit_complex_enum_type<T>(case_key: &String, content_key: &Option<String>) -> String
where
    T: EnumReflectionType,
{
    let cases_union = T::generate_cases_union();
    let case_keys_const = T::generate_case_key_const();
    let union_types = T::generate_union_types(&case_key, &content_key, T::inflection());
    let union_type = T::generate_union_schema();

    // Generate case type

    format!(
        r#"
{cases_union}

{case_keys_const}
{union_types}
{union_type}
"#
    )
}

trait EnumTypeBridge: EnumReflectionType {
    fn case_type_name() -> String {
        format!("{}Case", Self::name())
    }

    fn case_key_const_name() -> String {
        format!("{}CaseKey", Self::name())
    }

    fn generate_cases_union() -> String {
        let mut case_values = vec![];
        for case in Self::cases() {
            case_values.push(format!(r#""{name}""#, name = case.name));
        }

        let case_values = case_values.join("\n  | ");

        let name = Self::case_type_name();
        let cases = case_values;

        ts_string! {
            export type #name = #cases;
        }
    }

    fn generate_case_key_const() -> String {
        let mut case_values = String::new();

        case_values.push_str("\n  ");
        for case in Self::cases() {
            case_values.push_str(&format!(r#"{name}: "{name}""#, name = case.name));
            case_values.push_str(",\n  ");
        }

        let name = Self::case_key_const_name();
        let cases = case_values;

        ts_string! {
            export const #name = {
                #cases
            };
        }
    }

    fn generate_union_types(
        case_key: &String,
        content_key: &Option<String>,
        inflection: Inflection,
    ) -> String {
        let mut result = String::new();

        for case in Self::cases() {
            result.push_str(
                Self::generate_union_type(&case, &case_key, &content_key, inflection).as_str(),
            )
        }

        result
    }

    fn generate_union_type(
        case: &EnumCase,
        case_key: &String,
        content_key: &Option<String>,
        _inflection: Inflection,
    ) -> String {
        let case_type_name = union_case_type_name(case, Self::name());
        // let id = Self::case_id(case);
        let id = &case.name;

        let additional_fields = match &case.type_ {
            type_reflect_core::TypeFieldsDefinition::Unit => {
                return format!(
                    r#"
        export type {case_type_name} = "{id}";
                    "#
                )
            }
            type_reflect_core::TypeFieldsDefinition::Tuple(inner) => {
                let content_key = match content_key {
                    Some(content_key) => content_key,
                    None => {
                        //TODO: make this a localized Syn error
                        panic!("Content key required on enums containing at least one tuple-type variant.")
                    }
                };
                if inner.len() == 1 {
                    let type_ = to_ts_type(&inner[0]);
                    format!(
                        r#"{content_key}: {type_}"#,
                        type_ = type_,
                        content_key = content_key,
                    )
                } else {
                    let tuple_items: Vec<String> =
                        inner.into_iter().map(|item| to_ts_type(&item)).collect();
                    let tuple_items: String = tuple_items.join(",\n        ");

                    format!(
                        r#"{content_key}: [
    {tuple_items}
  ]"#,
                        tuple_items = tuple_items,
                        content_key = content_key,
                    )
                }
            }
            type_reflect_core::TypeFieldsDefinition::Named(inner) => {
                let struct_items = type_fields::named_fields(inner, case.inflection);

                match content_key {
                    Some(content_key) => format!(
                        r#"{content_key}: {{
    {struct_items}
  }}"#,
                        struct_items = struct_items,
                        content_key = content_key,
                    ),
                    None => struct_items,
                }
            }
        };
        format!(
            r#"
export type {case_type_name} = {{
  {case_key}: "{id}",
  {additional_fields}
}};
            "#
        )
    }

    fn generate_union_schema() -> String {
        let cases: Vec<String> = Self::cases()
            .into_iter()
            .map(|case| union_case_type_name(&case, Self::name()))
            .collect();

        let cases = cases.join("\n    | ");

        format!(
            r#"
export type {name} = {cases};
            "#,
            cases = cases,
            name = Self::name()
        )
    }
}

fn union_case_type_name(case: &EnumCase, parent_name: &str) -> String {
    format!("{}Case{}", parent_name, case.name)
}

impl<T> EnumTypeBridge for T where T: EnumReflectionType {}
