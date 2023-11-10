mod common;

use anyhow::Result;
use common::*;

use serde::{Deserialize, Serialize};
use type_reflect::*;

#[derive(Reflect, Serialize, Deserialize)]
pub struct Foo {
    pub x: f32,
}

#[test]
fn test_validate_valid() -> Result<()> {
    let output = init_path("test_simple_struct", "test_validate_valid");

    let post = r#"
assertDoesNotThrow(()=>{
    Foo.tryValidate({ x: 7 });
},
`{ x: 7 } should be a valid value of Foo`
);
    "#;

    export_types!(
        types: [ Foo ],
        destinations: [(
            output.ts_path(),
            prefix: TESTING_PREFIX,
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

#[test]
fn test_validate_invalid() -> Result<()> {
    let output = init_path("test_simple_struct", "test_validate_invalid");

    let post = r#"
assertThrows(()=>{
    Foo.tryValidate({ x: { a: 1 } });
},
`{ x: { a: 1 } } should be an invalid value of Foo`
);
    "#;

    export_types!(
        types: [ Foo ],
        destinations: [(
            output.ts_path(),
            prefix: TESTING_PREFIX,
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


#[test]
fn test_parse_valid() -> Result<()> {
    let output = init_path("test_simple_struct", "test_parse_valid");

    let post = r#"
assertDoesNotThrow(()=>{
    Foo.tryParse(`{ "x": 42 }`);
},
`'{ "x": 42 }' should be a valid value of Foo`
);
    "#;

    export_types!(
        types: [ Foo ],
        destinations: [(
            output.ts_path(),
            prefix: TESTING_PREFIX,
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

#[test]
fn test_parse_invalid() -> Result<()> {
    let output = init_path("test_simple_struct", "test_parse_invalid");

    let post = r#"
assertThrows(()=>{
    Foo.tryParse(`{ "x": "42" }`);
},
`'{ "x": "42" }' should be an invalid value of Foo`
);
    "#;

    export_types!(
        types: [ Foo ],
        destinations: [(
            output.ts_path(),
            prefix: TESTING_PREFIX,
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
