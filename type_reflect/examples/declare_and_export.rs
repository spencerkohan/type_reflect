use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use type_reflect::*;
use type_reflect::{export_types, Reflect};

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

#[derive(Debug, Reflect, Serialize, Deserialize)]
enum SimpleEnumsExample {
    Foo,
}

#[derive(Debug, Reflect, Serialize, Deserialize)]
struct DependantTypeExample {
    foo: SimpleEnumsExample,
}

fn main() {
    export_types! {
        types: [
            SDParameters,
            SimpleEnumsExample,
            Status,
        ],
        destinations: [
            Zod("./type_reflect/example_output/zod.ts"),
            Rust("./type_reflect/example_output/rust.rs"),
        ]
    }
    .unwrap();
}

//
