use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
enum SimpleEnumsExample {
    Foo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
