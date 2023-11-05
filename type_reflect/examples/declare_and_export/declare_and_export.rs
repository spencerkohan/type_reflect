use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use type_reflect::*;
use type_reflect::{export_types, Reflect};

// Here we declare a simple struct type with Reflect
// the serde(rename_all) attribute will rename the keys to
// camel case, both for the JSON representation, and for
// the Zod schemas when they are exported
#[derive(Debug, Reflect, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SDParameters {
    prompt: String,
    negative_prompt: Option<String>,
    cfg_scale: f32,
    step_count: u32,
    seed: u64,
    images: u32,
    results: Vec<String>,
    headers: HashMap<String, String>,
}

// Here we declare an enum wiht associated values.
// The `tag` attribute is required for all enums
// with associated data and in this case the `data`
// tag is also required (by serde) since we have
// tuple-typed enum variants
#[derive(Debug, Reflect, Serialize, Deserialize)]
#[serde(tag = "_case", content = "data")]
enum Status {
    Initial,
    #[serde(rename_all = "camelCase")]
    InProgress {
        progress: f32,
        should_convert: bool,
    },
    Complete {
        urls: Vec<String>,
    },
    Double(i32, f32),
    Single(i32),
}

// Here we have a simple enum type
#[derive(Debug, Reflect, Serialize, Deserialize)]
enum SimpleEnumsExample {
    Foo,
}

type AliasedEnum = SimpleEnumsExample;

// And here we have an example of a type which depends
// on a declared type, rather than primitive types
#[derive(Debug, Reflect, Serialize, Deserialize)]
struct DependantTypeExample {
    foo: SimpleEnumsExample,
}

#[derive(Debug, Serialize, Deserialize)]
struct Bar {}

#[derive(Debug, Reflect, Serialize, Deserialize)]
struct Foo {
    bar: Bar,
}

fn main() {
    // When the example is run, we export the specified
    // types to both a Zod target, and a Rust target
    export_types! {
        types: [
            SDParameters,
            SimpleEnumsExample,
            Status,
        ],
        destinations: [
            TypeScript(
                "./type_reflect/examples/declare_and_export/output/type_script.ts"
                tab_size: 2,
            ),
            Zod("./type_reflect/examples/declare_and_export/output/zod.ts"),
            // With a prefix arg, it's possible to add additional arbitrary
            // content to the output file.  So for instance this might be used
            // to add extra import statements for dependencies required by the
            // outputed type
            Rust(
                "./type_reflect/examples/declare_and_export/output/rust.rs",
                prefix: r#"// We add an extra comment here"#
            ),
            (
                "./type_reflect/examples/declare_and_export/output/multi.ts",
                emitters: [
                    TypeScript(),
                    Zod(),
                ]
            ),
        ]
    }
    .unwrap();

    export_types! {
        types: [
            Foo,
        ],
        destinations: [
            TypeScript(
                "./type_reflect/examples/declare_and_export/output/type_2.ts"
                prefix: "import { Bar } from './bar.ts'",
                tab_size: 2,

            ),
        ]
    }
    .unwrap();
}
