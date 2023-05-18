use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json;

use type_reflect::*;
use type_reflect::{export_types, Reflect};

// First version: use Rust types
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

// #[derive(Debug, Reflect, Serialize, Deserialize)]
// struct Foo2 {
//     name: String,
//     id: u32,
//     value: f64,
// }

// #[derive(Debug, Reflect)]
// struct SDMessage {
//     id: String,
//     params: SDParameters,
//     sqs_handle: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct Foo(String, i32);

#[derive(Debug, Reflect, Serialize, Deserialize)]
#[serde(tag = "_case", content = "data")]
enum Status {
    Initial,
    InProgress { progress: f32 },
    Complete { urls: Vec<String> },
    // Foo(i32, f32),
}

#[derive(Debug, Reflect, Serialize, Deserialize)]
enum SimpleEnumsExample {
    Foo,
}

#[derive(Debug, Reflect, Serialize, Deserialize)]
struct DependantTypeExample {
    foo: SimpleEnumsExample,
}

// Ideal version: use a small declarative DSL for type definitions
// shared_types! {
//     enum Status {
//         Initial,
//         Progress,
//         Complete {
//             urls: [String]
//         }
//     }

//     struct SDParams {
//         prompt: String,
//         negative_prompt: String? = None,
//         cfg_scale: f32 in 0...15 = 7.5,
//         steps: u32 in 1...256 = 25
//     }

//     struct Record {
//         status: Status,
//         params: SDParams
//     }
// }

fn main() {
    // let my_struct = Status::Foo(42, 69.0);

    // // Serialize `my_struct` to a JSON string.
    // let json = serde_json::to_string(&my_struct).unwrap();
    // println!("{}", json);

    let my_struct = Status::Initial;

    // Serialize `my_struct` to a JSON string.
    let json = serde_json::to_string(&my_struct).unwrap();
    println!("{}", json);

    export_types! {
        types: [
            SDParameters,
            SimpleEnumsExample,
            Status,
            // Foo
        ],
        destinations: [
            Zod("./example/output/zod.ts"),
            Rust("./example/output/rust.rs"),
        ]
    }
    .unwrap();
}

//
