use type_reflect::*;
use type_reflect::{export_types, Reflect};

// First version: use Rust types
#[derive(Debug, Reflect)]
struct SDParameters {
    prompt: String,
    // negative_prompt: Option<String>,
    cfg_scale: f32,
    step_count: u32,
    seed: u64,
    images: u32,
}

// #[derive(Debug, Reflect)]
// struct SDMessage {
//     id: String,
//     params: SDParameters,
//     sqs_handle: String,
// }

// #[derive(Debug, Reflect)]
// enum Status {
//     Initial,
//     InProgress { progress: f32 },
//     Complete { urls: Vec<String> },
// }

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
    export_types! {
        types: [
            SDParameters,
        ],
        destinations: [
            Zod("./example/output/zod.ts")
        ]
    }
}

// Rust("./output/rust.rs"),
