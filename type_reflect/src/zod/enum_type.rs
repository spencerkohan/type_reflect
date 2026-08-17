use type_reflect_core::{EnumCase, EnumType, Inflection, TypeFieldsDefinition};

use crate::EnumReflectionType;

use super::{named_fields, to_zod_type};

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
        EnumType::ExternallyTagged => emit_externally_tagged_enum_type::<T>(),
        EnumType::Untagged => emit_untagged_enum_type::<T>(),
    }
}

/// Serde's default (external) representation: unit cases serialize to the
/// case-name string, tuple/struct cases to `{ "CASE": content }`.
/// Mirrors the externally-tagged model of the TypeScript / TSValidation emitters.
fn emit_externally_tagged_enum_type<T>() -> String
where
    T: EnumReflectionType,
{
    let name = T::name();
    let inflection = T::inflection();
    let members: Vec<String> = T::cases()
        .iter()
        .map(|case| externally_tagged_case_schema(case, inflection))
        .collect();

    // `z.union` requires at least two members; emit the bare schema otherwise.
    let schema = if members.len() == 1 {
        members[0].clone()
    } else {
        let members = members.join(",\n");
        format!(
            r#"z.union([
{members}
])"#
        )
    };

    let schema_name = format!("{}Schema", name);

    format!(
        r#"
export const {schema_name} = {schema};
export type {name} = z.infer<typeof {schema_name}>
"#
    )
}

fn externally_tagged_case_schema(case: &EnumCase, inflection: Inflection) -> String {
    let key = case.serialized_name(inflection);
    match &case.type_ {
        TypeFieldsDefinition::Unit => format!(r#"z.literal({key:?})"#),
        _ => format!(r#"z.object({{ "{key}": {} }})"#, bare_content_schema(case)),
    }
}

/// Serde's `#[serde(untagged)]` representation: each case serializes to its
/// bare content (struct -> `{ fields }`, tuple -> `[a, b]`, unit -> `null`).
fn emit_untagged_enum_type<T>() -> String
where
    T: EnumReflectionType,
{
    let name = T::name();
    let members: Vec<String> = T::cases()
        .iter()
        .map(|case| bare_content_schema(case))
        .collect();

    // `z.union` requires at least two members; emit the bare schema otherwise.
    let schema = if members.len() == 1 {
        members[0].clone()
    } else {
        let members = members.join(",\n");
        format!(
            r#"z.union([
{members}
])"#
        )
    };

    let schema_name = format!("{}Schema", name);

    format!(
        r#"
export const {schema_name} = {schema};
export type {name} = z.infer<typeof {schema_name}>
"#
    )
}

/// The Zod schema for a case's bare content (no case key).
fn bare_content_schema(case: &EnumCase) -> String {
    match &case.type_ {
        TypeFieldsDefinition::Unit => "z.null()".to_string(),
        TypeFieldsDefinition::Tuple(items) => {
            if items.len() == 1 {
                to_zod_type(&items[0])
            } else {
                let items: Vec<String> = items.iter().map(|t| to_zod_type(t)).collect();
                format!("z.tuple([{}])", items.join(", "))
            }
        }
        TypeFieldsDefinition::Named(fields) => format!(
            "z.object({{ {} }})",
            named_fields(fields, case.inflection)
        ),
    }
}

fn emit_simple_enum_type<T>() -> String
where
    T: EnumReflectionType,
{
    let inflection = T::inflection();
    let simple_cases: String = T::cases()
        .into_iter()
        .map(|case| {
            format!(
                r#"    {name} = "{serialized}",
"#,
                name = case.name,
                serialized = case.serialized_name(inflection)
            )
        })
        .collect();

    let schema_name = T::union_schema_name();
    let schema_cases: String = T::cases()
        .into_iter()
        .map(|case| {
            format!(
                "    {enum_name}.{case_name},\n",
                enum_name = T::name(),
                case_name = case.name
            )
        })
        .collect();

    format!(
        r#"
export enum {name} {{
{simple_cases}}}

export const {schema_name} = z.enum([
{schema_cases}])
"#,
        name = T::name(),
        simple_cases = simple_cases,
        schema_name = schema_name,
        schema_cases = schema_cases
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

    // let members = enum_cases(&T::cases());

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
        let inflection = Self::inflection();
        for case in Self::cases() {
            let serialized = case.serialized_name(inflection);
            case_values
                .push_str(format!(r#"    {name} = "{serialized}""#, name = case.name).as_str());
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
        let schema_name = union_type_name(case, Self::name());
        let id = Self::case_id(case);
        // tag/content keys are user-supplied strings: quote them if they
        // aren't valid identifiers
        let case_key = crate::ts_key(case_key);
        let content_key = content_key.as_ref().map(|k| crate::ts_key(k));

        let additional_fields = match &case.type_ {
            type_reflect_core::TypeFieldsDefinition::Unit => String::new(),
            type_reflect_core::TypeFieldsDefinition::Tuple(inner) => {
                let content_key = match content_key {
                    Some(content_key) => content_key,
                    None => {
                        // Rejected at derive time (see EnumDef::new); only
                        // reachable through hand-written impls.
                        panic!("Content key required on enums containing at least one tuple-type variant.")
                    }
                };
                if inner.len() == 1 {
                    let type_ = to_zod_type(&inner[0]);
                    format!(
                        r#"    {content_key}: {type_}"#,
                        type_ = type_,
                        content_key = content_key,
                    )
                } else {
                    let tuple_items: String = inner
                        .into_iter()
                        .map(|item| format!("        {},\n", to_zod_type(&item)))
                        .collect();

                    format!(
                        r#"    {content_key}: z.tuple([
    {tuple_items}    ])"#,
                        tuple_items = tuple_items,
                        content_key = content_key,
                    )
                }
            }
            type_reflect_core::TypeFieldsDefinition::Named(inner) => {
                let struct_items: String = inner
                    .into_iter()
                    .map(|item| {
                        format!(
                            "    {}: {},\n",
                            crate::ts_key(&item.serialized_name(case.inflection)),
                            to_zod_type(&item.type_)
                        )
                    })
                    .collect();

                match content_key {
                    Some(content_key) => format!(
                        r#"    {content_key}: z.object({{
    {struct_items}    }})"#,
                        struct_items = struct_items,
                        content_key = content_key,
                    ),
                    None => struct_items,
                }
            }
        };
        format!(
            r#"
export const {schema_name} = z.object({{
    {case_key}: z.literal({id}),
{additional_fields}}});
export type {name} = z.infer<typeof {schema_name}>
            "#,
            schema_name = schema_name,
            name = format!("{}Case{}", Self::name(), case.name),
            case_key = case_key,
            id = id,
            additional_fields = additional_fields
        )
    }

    fn union_schema_name() -> String {
        format!("{}Schema", Self::name())
    }

    fn generate_union_schema() -> String {
        let schema_name = Self::union_schema_name();
        let cases = Self::cases();
        // `z.union` requires at least two members; emit the single case
        // schema directly otherwise.
        let schema = if cases.len() == 1 {
            union_type_name(&cases[0], Self::name())
        } else {
            let mut case_schemas = String::new();

            for case in &cases {
                case_schemas.push_str(format!("    {},\n", union_type_name(case, Self::name())).as_str());
            }

            format!(
                r#"z.union([
{case_schemas}])"#
            )
        };

        format!(
            r#"
export const {schema_name} = {schema};
export type {name} = z.infer<typeof {schema_name}>
            "#,
            schema = schema,
            schema_name = schema_name,
            name = Self::name()
        )
    }
}

fn union_type_name(case: &EnumCase, parent_name: &str) -> String {
    format!("{}Case{}Schema", parent_name, case.name)
}

impl<T> EnumTypeBridge for T where T: EnumReflectionType {}
