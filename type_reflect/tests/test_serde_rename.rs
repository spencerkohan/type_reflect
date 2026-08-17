//! Gap investigation: serde rename attributes vs. the types emitted by the
//! TypeScript, TSValidation and Zod emitters.
//!
//! Ground truth is what `serde_json` actually serializes. For every rename
//! scenario we:
//!   1. serialize representative values with serde,
//!   2. emit the type with the TypeScript + TSValidation emitters into one
//!      `.ts` file and with the Zod emitter into a sibling `.zod.ts` file
//!      (separate files because both families emit a `Shape`/`Pet` symbol and
//!      would collide in one file),
//!   3. assert that the emitted validators accept the serde output.
//!
//! Tests that FAIL document a gap between serde's serialization output and
//! the emitted types. They are expected to fail until the macro/emitters are
//! fixed.

mod common;

use common::*;
use std::fs;

use anyhow::Result;
use serde::Serialize;
use type_reflect::*;

const SCOPE: &str = "test_serde_rename";

/// Build the jest body: one describe block per serde-serialized value,
/// checking the TSValidation `parse` and (optionally) the Zod schema.
fn jest_body(validator: &str, zod_schema: Option<&str>, jsons: &[String]) -> String {
    let mut body = String::new();
    for (i, json) in jsons.iter().enumerate() {
        let zod_check = match zod_schema {
            Some(schema) => format!(
                r#"
  it('Zod schema accepts serde output', () => {{
    expect(() => {schema}.parse(JSON.parse(serde{i}))).not.toThrow();
  }});
"#,
            ),
            None => String::new(),
        };
        body.push_str(&format!(
            r#"
const serde{i} = {json};

describe('serde output #{i}', () => {{
  it('TSValidation accepts serde output', () => {{
    expect(() => {validator}.parse(serde{i})).not.toThrow();
  }});
  {zod_check}
}});
"#,
        ));
    }
    body
}

/// Write the jest test (the types must already have been emitted with
/// `export_types!`) and run it. Note: does NOT call `init_path` again, as
/// that would clean (delete) the just-emitted type files.
fn run_jest(
    name: &str,
    ts_imports: &str,
    zod_imports: Option<&str>,
    validator: &str,
    zod_schema: Option<&str>,
    jsons: Vec<String>,
) -> Result<()> {
    let ts_path = std::path::PathBuf::from(OUTPUT_DIR)
        .join("src")
        .join(SCOPE)
        .join(format!("{}.ts", name));
    let jest_path = ts_path.with_extension("test.ts");

    let jsons: Vec<String> = jsons
        .into_iter()
        .map(|json| serde_json::to_string(&json))
        .collect::<Result<_, _>>()?;

    let imports = format!("import {{ {ts_imports} }} from './{name}';");
    let zod_import = match zod_imports {
        Some(z) => format!("\nimport {{ {z} }} from './{name}.zod';"),
        None => String::new(),
    };

    let jest = format!("{imports}{zod_import}\n{}", jest_body(validator, zod_schema, &jsons));
    fs::write(&jest_path, jest)?;

    println!("Running jest against {}", jest_path.display());
    run_command(
        OUTPUT_DIR,
        format!("yarn jest {}", jest_path.to_str().unwrap()).as_str(),
    )
}

fn run_command(dir: &str, command: &str) -> Result<()> {
    let mut parts = command.split_whitespace();
    let command = parts.next().expect("no command given");
    let args = parts.collect::<Vec<&str>>();

    let mut child = std::process::Command::new(command)
        .args(&args)
        .current_dir(dir)
        .spawn()?;

    let status = child.wait()?;

    if !status.success() {
        anyhow::bail!("Command failed: {}", command);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Simple (all-unit) enums
// ---------------------------------------------------------------------------

mod simple_rename_all {
    use super::*;

    #[derive(Reflect, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub enum Pet {
        DogBreed,
        CatType,
    }

    #[test]
    fn serde_output_is_accepted() -> Result<()> {
        let output = init_path(SCOPE, "simple_rename_all");
        export_types!(
            types: [ Pet ],
            destinations: [
                ( output.ts_path(), emitters: [ TypeScript(), TSValidation() ] ),
                (
                    output.ts_path().with_file_name("simple_rename_all.zod.ts"),
                    emitters: [ Zod() ],
                ),
            ]
        )?;
        let jsons = [Pet::DogBreed, Pet::CatType]
            .iter()
            .map(|v| serde_json::to_string(v).unwrap())
            .collect();
        run_jest(
            "simple_rename_all",
            "Pet",
            Some("PetSchema"),
            "Pet",
            Some("PetSchema"),
            jsons,
        )
    }
}

mod simple_case_rename {
    use super::*;

    #[derive(Reflect, Serialize)]
    pub enum Pet {
        #[serde(rename = "custom_dog")]
        DogBreed,
        CatType,
    }

    #[test]
    fn serde_output_is_accepted() -> Result<()> {
        let output = init_path(SCOPE, "simple_case_rename");
        export_types!(
            types: [ Pet ],
            destinations: [
                ( output.ts_path(), emitters: [ TypeScript(), TSValidation() ] ),
                (
                    output.ts_path().with_file_name("simple_case_rename.zod.ts"),
                    emitters: [ Zod() ],
                ),
            ]
        )?;
        let jsons = [Pet::DogBreed, Pet::CatType]
            .iter()
            .map(|v| serde_json::to_string(v).unwrap())
            .collect();
        run_jest(
            "simple_case_rename",
            "Pet",
            Some("PetSchema"),
            "Pet",
            Some("PetSchema"),
            jsons,
        )
    }
}

// ---------------------------------------------------------------------------
// Complex (tagged) enums
// ---------------------------------------------------------------------------

mod complex_external_rename_all {
    use super::*;

    // Note: split into two `#[serde]` attributes because the macro's serde
    // parser drops the whole attribute if it contains a key it doesn't know
    // (see tag_and_rename_all_single_attr_classification).
    #[derive(Reflect, Serialize)]
    #[serde(tag = "_case")]
    #[serde(rename_all = "kebab-case")]
    pub enum Shape {
        Circle { radius: f32 },
        Square { side: f32 },
    }

    #[test]
    fn serde_output_is_accepted() -> Result<()> {
        let output = init_path(SCOPE, "complex_external_rename_all");
        export_types!(
            types: [ Shape ],
            destinations: [
                (
                    output.ts_path(),
                    emitters: [ TypeScript(), TSValidation() ],
                ),
                (
                    output
                        .ts_path()
                        .with_file_name("complex_external_rename_all.zod.ts"),
                    emitters: [ Zod() ],
                ),
            ]
        )?;
        let jsons = [
            &Shape::Circle { radius: 5.0 },
            &Shape::Square { side: 2.0 },
        ]
        .iter()
        .map(|v| serde_json::to_string(v).unwrap())
        .collect();
        run_jest(
            "complex_external_rename_all",
            "Shape",
            Some("ShapeSchema"),
            "Shape",
            Some("ShapeSchema"),
            jsons,
        )
    }
}

mod complex_internal_rename_all {
    use super::*;

    #[derive(Reflect, Serialize)]
    #[serde(tag = "_tag")]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum Color {
        DarkRed { shade: f32 },
        Blue,
    }

    #[test]
    fn serde_output_is_accepted() -> Result<()> {
        let output = init_path(SCOPE, "complex_internal_rename_all");
        export_types!(
            types: [ Color ],
            destinations: [
                (
                    output.ts_path(),
                    emitters: [ TypeScript(), TSValidation() ],
                ),
                (
                    output
                        .ts_path()
                        .with_file_name("complex_internal_rename_all.zod.ts"),
                    emitters: [ Zod() ],
                ),
            ]
        )?;
        let jsons = [
            &Color::DarkRed { shade: 0.5 },
            &Color::Blue,
        ]
        .iter()
        .map(|v| serde_json::to_string(v).unwrap())
        .collect();
        run_jest(
            "complex_internal_rename_all",
            "Color",
            Some("ColorSchema"),
            "Color",
            Some("ColorSchema"),
            jsons,
        )
    }
}

mod complex_external_case_rename {
    use super::*;

    #[derive(Reflect, Serialize)]
    #[serde(tag = "_case")]
    pub enum Shape {
        #[serde(rename = "renamed_circle")]
        Circle { radius: f32 },
        // second variant: a single-variant zod file emits z.union([one]),
        // which doesn't typecheck (see report); unrelated to renames.
        Square { side: f32 },
    }

    #[test]
    fn serde_output_is_accepted() -> Result<()> {
        let output = init_path(SCOPE, "complex_external_case_rename");
        export_types!(
            types: [ Shape ],
            destinations: [
                (
                    output.ts_path(),
                    emitters: [ TypeScript(), TSValidation() ],
                ),
                (
                    output
                        .ts_path()
                        .with_file_name("complex_external_case_rename.zod.ts"),
                    emitters: [ Zod() ],
                ),
            ]
        )?;
        let jsons = [
            &Shape::Circle { radius: 5.0 },
            &Shape::Square { side: 2.0 },
        ]
        .iter()
        .map(|v| serde_json::to_string(v).unwrap())
        .collect();
        run_jest(
            "complex_external_case_rename",
            "Shape",
            Some("ShapeSchema"),
            "Shape",
            Some("ShapeSchema"),
            jsons,
        )
    }
}

mod complex_external_field_rename {
    use super::*;

    #[derive(Reflect, Serialize)]
    #[serde(tag = "_case")]
    pub enum Shape {
        Circle {
            #[serde(rename = "custom_radius")]
            radius: f32,
        },
        Square { side: f32 },
    }

    #[test]
    fn serde_output_is_accepted() -> Result<()> {
        let output = init_path(SCOPE, "complex_external_field_rename");
        export_types!(
            types: [ Shape ],
            destinations: [
                (
                    output.ts_path(),
                    emitters: [ TypeScript(), TSValidation() ],
                ),
                (
                    output
                        .ts_path()
                        .with_file_name("complex_external_field_rename.zod.ts"),
                    emitters: [ Zod() ],
                ),
            ]
        )?;
        let jsons = [
            &Shape::Circle { radius: 5.0 },
            &Shape::Square { side: 2.0 },
        ]
        .iter()
        .map(|v| serde_json::to_string(v).unwrap())
        .collect();
        run_jest(
            "complex_external_field_rename",
            "Shape",
            Some("ShapeSchema"),
            "Shape",
            Some("ShapeSchema"),
            jsons,
        )
    }
}

mod complex_external_variant_rename_all {
    use super::*;

    #[derive(Reflect, Serialize)]
    #[serde(tag = "_case")]
    pub enum Shape {
        #[serde(rename_all = "camelCase")]
        Circle {
            my_radius: f32,
            outer_side: f32,
        },
        Square { plain_side: f32 },
    }

    #[test]
    fn serde_output_is_accepted() -> Result<()> {
        let output = init_path(SCOPE, "complex_external_variant_rename_all");
        export_types!(
            types: [ Shape ],
            destinations: [
                (
                    output.ts_path(),
                    emitters: [ TypeScript(), TSValidation() ],
                ),
                (
                    output
                        .ts_path()
                        .with_file_name("complex_external_variant_rename_all.zod.ts"),
                    emitters: [ Zod() ],
                ),
            ]
        )?;
        let jsons = [
            &Shape::Circle {
                my_radius: 5.0,
                outer_side: 2.0,
            },
            &Shape::Square { plain_side: 3.0 },
        ]
        .iter()
        .map(|v| serde_json::to_string(v).unwrap())
        .collect();
        run_jest(
            "complex_external_variant_rename_all",
            "Shape",
            Some("ShapeSchema"),
            "Shape",
            Some("ShapeSchema"),
            jsons,
        )
    }
}

mod tag_and_rename_all_single_attr_classification {
    use super::*;

    // serde allows `#[serde(tag = "_case", rename_all = "kebab-case")]` on a
    // single attribute. The macro's serde parser rejects the whole attribute
    // at the unknown `rename_all` key and silently drops the tag, so the enum
    // is misclassified as Untagged (default external) instead of Complex.
    #[derive(Reflect, Serialize)]
    #[serde(tag = "_case", rename_all = "kebab-case")]
    pub enum Shape {
        Circle { radius: f32 },
    }

    #[test]
    fn enum_is_classified_as_complex() {
        assert!(
            matches!(Shape::enum_type(), EnumType::Complex { .. }),
            "enum with #[serde(tag = ..., rename_all = ...)] must classify as Complex, got {:?}",
            Shape::enum_type()
        );
    }
}

mod complex_combo_rename_alls {
    use super::*;

    // enum-level rename_all renames case names, variant-level rename_all
    // renames the variant's fields; serde applies both, independently.
    #[derive(Reflect, Serialize)]
    #[serde(tag = "_case")]
    #[serde(rename_all = "snake_case")]
    pub enum Shape {
        #[serde(rename_all = "camelCase")]
        BigCircle { my_radius: f32 },
        SmallSquare { plain_side: f32 },
    }

    #[test]
    fn serde_output_is_accepted() -> Result<()> {
        let output = init_path(SCOPE, "complex_combo_rename_alls");
        export_types!(
            types: [ Shape ],
            destinations: [
                (
                    output.ts_path(),
                    emitters: [ TypeScript(), TSValidation() ],
                ),
                (
                    output
                        .ts_path()
                        .with_file_name("complex_combo_rename_alls.zod.ts"),
                    emitters: [ Zod() ],
                ),
            ]
        )?;
        let jsons = [
            &Shape::BigCircle { my_radius: 5.0 },
            &Shape::SmallSquare { plain_side: 3.0 },
        ]
        .iter()
        .map(|v| serde_json::to_string(v).unwrap())
        .collect();
        run_jest(
            "complex_combo_rename_alls",
            "Shape",
            Some("ShapeSchema"),
            "Shape",
            Some("ShapeSchema"),
            jsons,
        )
    }
}

// ---------------------------------------------------------------------------
// Structs
// ---------------------------------------------------------------------------

mod struct_rename_all {
    use super::*;

    #[derive(Reflect, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Rectangle {
        my_width: f32,
        my_height: f32,
    }

    #[test]
    fn serde_output_is_accepted() -> Result<()> {
        let output = init_path(SCOPE, "struct_rename_all");
        export_types!(
            types: [ Rectangle ],
            destinations: [
                (
                    output.ts_path(),
                    emitters: [ TypeScript(), TSValidation() ],
                ),
                (
                    output
                        .ts_path()
                        .with_file_name("struct_rename_all.zod.ts"),
                    emitters: [ Zod() ],
                ),
            ]
        )?;
        let jsons = [Rectangle {
            my_width: 1.0,
            my_height: 2.0,
        }]
        .iter()
        .map(|v| serde_json::to_string(v).unwrap())
        .collect();
        run_jest(
            "struct_rename_all",
            "Rectangle",
            Some("RectangleSchema"),
            "Rectangle",
            Some("RectangleSchema"),
            jsons,
        )
    }
}

mod struct_field_rename {
    use super::*;

    #[derive(Reflect, Serialize)]
    pub struct Rectangle {
        #[serde(rename = "custom_width")]
        width: f32,
        height: f32,
    }

    #[test]
    fn serde_output_is_accepted() -> Result<()> {
        let output = init_path(SCOPE, "struct_field_rename");
        export_types!(
            types: [ Rectangle ],
            destinations: [
                (
                    output.ts_path(),
                    emitters: [ TypeScript(), TSValidation() ],
                ),
                (
                    output
                        .ts_path()
                        .with_file_name("struct_field_rename.zod.ts"),
                    emitters: [ Zod() ],
                ),
            ]
        )?;
        let jsons = [Rectangle {
            width: 1.0,
            height: 2.0,
        }]
        .iter()
        .map(|v| serde_json::to_string(v).unwrap())
        .collect();
        run_jest(
            "struct_field_rename",
            "Rectangle",
            Some("RectangleSchema"),
            "Rectangle",
            Some("RectangleSchema"),
            jsons,
        )
    }
}

// ---------------------------------------------------------------------------
// Untagged enums
// ---------------------------------------------------------------------------

mod untagged_unit_rename_all {
    use super::*;

    // The non-unit case is only there to make the enum classify as Untagged.
    #[derive(Reflect, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum Cmd {
        Noop,
        ReadAll,
        DoIt { count: u32 },
    }

    #[test]
    fn serde_output_is_accepted() -> Result<()> {
        let output = init_path(SCOPE, "untagged_unit_rename_all");
        export_types!(
            types: [ Cmd ],
            destinations: [
                ( output.ts_path(), emitters: [ TypeScript(), TSValidation() ] ),
            ]
        )?;
        let jsons = [Cmd::Noop, Cmd::ReadAll]
            .iter()
            .map(|v| serde_json::to_string(v).unwrap())
            .collect();
        run_jest("untagged_unit_rename_all", "Cmd", None, "Cmd", None, jsons)
    }
}

mod untagged_unit_case_rename {
    use super::*;

    #[derive(Reflect, Serialize)]
    pub enum Cmd {
        Noop,
        #[serde(rename = "read-all-items")]
        ReadAll,
        DoIt { count: u32 },
    }

    #[test]
    fn serde_output_is_accepted() -> Result<()> {
        let output = init_path(SCOPE, "untagged_unit_case_rename");
        export_types!(
            types: [ Cmd ],
            destinations: [
                ( output.ts_path(), emitters: [ TypeScript(), TSValidation() ] ),
            ]
        )?;
        let jsons = [Cmd::Noop, Cmd::ReadAll]
            .iter()
            .map(|v| serde_json::to_string(v).unwrap())
            .collect();
        run_jest(
            "untagged_unit_case_rename",
            "Cmd",
            None,
            "Cmd",
            None,
            jsons,
        )
    }
}

mod untagged_struct_representation {
    use super::*;

    // An enum without a `tag` attribute is serde's default (external)
    // representation: unit cases serialize to the case-name string, struct
    // cases to `{ "CASE_NAME": { fields } }`, newtype cases to
    // `{ "CASE_NAME": content }`. This is what the emitters call Untagged.
    #[derive(Reflect, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum Shape {
        #[serde(rename_all = "camelCase")]
        Circle { my_radius: f32 },
        Scale(f32),
    }

    #[test]
    fn serde_output_is_accepted() -> Result<()> {
        let output = init_path(SCOPE, "untagged_struct_representation");
        export_types!(
            types: [ Shape ],
            destinations: [
                ( output.ts_path(), emitters: [ TypeScript(), TSValidation() ] ),
            ]
        )?;
        let jsons = [
            &Shape::Circle { my_radius: 5.0 },
            &Shape::Scale(2.0),
        ]
        .iter()
        .map(|v| serde_json::to_string(v).unwrap())
        .collect();
        run_jest(
            "untagged_struct_representation",
            "Shape",
            None,
            "Shape",
            None,
            jsons,
        )
    }
}

mod untagged_kebab_case_key {
    use super::*;

    // kebab-case case names are used as object keys / property accesses in
    // the untagged emitters; this checks they produce valid TypeScript.
    #[derive(Reflect, Serialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum Shape {
        Noop,
        GetMax { limit: u32 },
    }

    #[test]
    fn serde_output_is_accepted() -> Result<()> {
        let output = init_path(SCOPE, "untagged_kebab_case_key");
        export_types!(
            types: [ Shape ],
            destinations: [
                ( output.ts_path(), emitters: [ TypeScript(), TSValidation() ] ),
            ]
        )?;
        let jsons = [Shape::Noop, Shape::GetMax { limit: 10 }]
            .iter()
            .map(|v| serde_json::to_string(v).unwrap())
            .collect();
        run_jest(
            "untagged_kebab_case_key",
            "Shape",
            None,
            "Shape",
            None,
            jsons,
        )
    }
}

mod kebab_struct_field {
    use super::*;

    // Non-identifier (kebab-case) field names must be quoted as object keys
    // and property accesses in all three emitters (this is the field-side
    // twin of untagged_kebab_case_key, which covers case keys).
    #[derive(Reflect, Serialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Size {
        min_width: u32,
    }

    #[test]
    fn serde_output_is_accepted() -> Result<()> {
        let output = init_path(SCOPE, "kebab_struct_field");
        export_types!(
            types: [ Size ],
            destinations: [
                ( output.ts_path(), emitters: [ TypeScript(), TSValidation() ] ),
                (
                    output.ts_path().with_file_name("kebab_struct_field.zod.ts"),
                    emitters: [ Zod() ],
                ),
            ]
        )?;
        let jsons = [Size { min_width: 5 }]
            .iter()
            .map(|v| serde_json::to_string(v).unwrap())
            .collect();
        run_jest(
            "kebab_struct_field",
            "Size",
            Some("SizeSchema"),
            "Size",
            Some("SizeSchema"),
            jsons,
        )
    }
}

mod serde_untagged_attribute {
    use super::*;

    // `#[serde(untagged)]` is a real serde attribute: the variant content
    // serializes bare (struct case -> `{ "my_radius": 5 }` with no case key,
    // unit case -> `null`). The macro's serde parser doesn't know the
    // `untagged` key, silently drops it, and emits the keyed (default
    // external) representation with `"Null"`-style string literals.
    #[derive(Reflect, Serialize)]
    #[serde(untagged)]
    pub enum Shape {
        Circle { my_radius: f32 },
        Null,
    }

    #[test]
    fn serde_output_is_accepted() -> Result<()> {
        let output = init_path(SCOPE, "serde_untagged_attribute");
        export_types!(
            types: [ Shape ],
            destinations: [
                ( output.ts_path(), emitters: [ TypeScript(), TSValidation() ] ),
                (
                    output.ts_path().with_file_name("serde_untagged_attribute.zod.ts"),
                    emitters: [ Zod() ],
                ),
            ]
        )?;
        let jsons = [
            &Shape::Circle { my_radius: 5.0 },
            &Shape::Null,
        ]
        .iter()
        .map(|v| serde_json::to_string(v).unwrap())
        .collect();
        run_jest(
            "serde_untagged_attribute",
            "Shape",
            Some("ShapeSchema"),
            "Shape",
            Some("ShapeSchema"),
            jsons,
        )
    }
}

mod zod_untagged_supported {
    use super::*;

    // The Zod emitter must support the untagged (default external) class:
    // unit cases -> string literal, struct cases -> { "CASE": { fields } }.
    #[derive(Reflect, Serialize)]
    pub enum Shape {
        Circle { radius: f32 },
        Null,
    }

    #[test]
    fn serde_output_is_accepted() -> Result<()> {
        let output = init_path(SCOPE, "zod_untagged");
        export_types!(
            types: [ Shape ],
            destinations: [
                ( output.ts_path(), emitters: [ TypeScript(), TSValidation() ] ),
                (
                    output
                        .ts_path()
                        .with_file_name("zod_untagged.zod.ts"),
                    emitters: [ Zod() ],
                ),
            ]
        )?;
        let jsons = [
            &Shape::Circle { radius: 5.0 },
            &Shape::Null,
        ]
        .iter()
        .map(|v| serde_json::to_string(v).unwrap())
        .collect();
        run_jest(
            "zod_untagged",
            "Shape",
            Some("ShapeSchema"),
            "Shape",
            Some("ShapeSchema"),
            jsons,
        )
    }
}
