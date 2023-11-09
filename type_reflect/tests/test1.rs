mod common;
use std::fs;

use anyhow::Result;
use common::*;

use serde::{Deserialize, Serialize};
use type_reflect::*;

#[derive(Reflect, Serialize, Deserialize)]
pub struct Foo {
    pub x: f32,
}

#[test]
fn test_filename() -> Result<()> {
    let output = init_path("test1");

    let post = r#"
let v1 = Foo.validate({ x: 7 });
console.log(`foo?: ${JSON.stringify(v1, null, 4)}`);

let v2 = Foo.parse(`{"x": 42}`);
console.log(`foo?: ${JSON.stringify(v2, null, 4)}`);

let v3 = Foo.parse(`{"x": "42"}`);
console.log(`foo?: ${JSON.stringify(v3, null, 4)}`);

let v4 = Foo.validate({ x: [1, 2, 3] });
console.log(`foo?: ${JSON.stringify(v4, null, 4)}`);


let v5 = Foo.tryValidate({ x: [1, 2, 3] });


    "#;

    export_types!(
        types: [ Foo ],
        destinations: [(
            output.ts_path(),
            postfix: post,
            emitters: [
                TypeScript(),
                TSValidation(),
                TSFormat(
                    tab_size: 2,
                    line_width: 80,
                ),
            ],
        )]
    )?;
    output.run_ts()
}
