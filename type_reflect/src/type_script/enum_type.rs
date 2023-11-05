use type_reflect_core::{EnumCase, EnumType, Inflection};

use crate::type_script::struct_type;
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
    let cases_enum = T::generate_cases_enum();
    let union_types = T::generate_union_types(&case_key, &content_key, T::inflection());
    let union_type = T::generate_union_schema();

    // Generate case type

    format!(
        r#"
{cases_enum}
{union_types}
{union_type}
"#,
        cases_enum = cases_enum,
        union_types = union_types,
        union_type = union_type
    )
}

trait EnumTypeBridge: EnumReflectionType {
    fn case_type_name() -> String {
        format!("{}Case", Self::name())
    }

    fn case_id(case: &EnumCase) -> String {
        format!("{}.{}", Self::case_type_name(), case.name)
    }

    fn generate_cases_enum() -> String {
        let mut case_values = String::new();
        for case in Self::cases() {
            case_values.push_str(format!(r#"  {name} = "{name}""#, name = case.name).as_str());
            case_values.push_str(",\n");
        }

        format!(
            r#"
export enum {name} {{
{cases}}}
"#,
            name = Self::case_type_name(),
            cases = case_values
        )
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
        let id = Self::case_id(case);

        let additional_fields = match &case.type_ {
            type_reflect_core::EnumCaseType::Simple => String::new(),
            type_reflect_core::EnumCaseType::Tuple(inner) => {
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
            type_reflect_core::EnumCaseType::Struct(inner) => {
                let struct_items = struct_type::struct_members(inner, case.inflection);

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
  {case_key}: {id},
  {additional_fields}
}};
            "#,
            case_type_name = case_type_name,
            case_key = case_key,
            id = id,
            additional_fields = additional_fields
        )
    }

    fn union_member_type_name() -> String {
        format!("{}", Self::name())
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
