use type_reflect::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SDParameters
{
    prompt : String, negative_prompt : Option < String >, cfg_scale : f32,
    step_count : u32, seed : u64, images : u32, results : Vec < String >,
    headers : HashMap < String, String >,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum SimpleEnumsExample { Foo, }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_case", content = "data")] enum Status
{
    Initial, InProgress { progress : f32 }, Complete
    { urls : Vec < String > },
}
