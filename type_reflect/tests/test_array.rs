mod common;

use anyhow::Result;
use common::*;

use serde::{Deserialize, Serialize};
use type_reflect::*;

pub const SCOPE: &'static str = "test_array";

#[derive(Reflect, Serialize, Deserialize)]
pub struct Bar {
    val: bool,
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct ArrayOfPrimitive {
    records: Vec<u32>,
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct ArrayOfType {
    records: Vec<Bar>,
}

#[test]
fn test_array_of_primitive() -> Result<()> {
    let output = init_path(SCOPE, "test_array_of_primitive");

    export_types!(
        types: [ ArrayOfPrimitive ],
        destinations: [(
            output.ts_path(),
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

    output.write_jest(
        "ArrayOfPrimitive",
        r#"

describe('Struct with Array of Primitives Validation', ()=>{

  it('validates an object: `{ records: [42, 7, 3, 21] }` which conforms to ArrayOfPrimitive', ()=>{
    expect(() => {
        ArrayOfPrimitive.tryValidate({ records: [42, 7, 3, 21] });
    }).not.toThrow();
  });

  it('validates an empty array: `{ records: [] }` which conforms to ArrayOfPrimitive', ()=>{
    expect(() => {
        ArrayOfPrimitive.tryValidate({ records: [] });
    }).not.toThrow();
  });

  it('throws an error validating an object: `{ records: [42, 7, "3", 21] }` which has one value not conforming to the type', ()=>{
    expect(() => {
        ArrayOfPrimitive.tryValidate({ records: [42, 7, "3", 21] })
    }).toThrow();
  });

})
    "#,
    )?;

    output.run_ts()
}

#[test]
fn test_nested_array() -> Result<()> {
    let output = init_path(SCOPE, "test_nested_array");

    export_types!(
        types: [ ArrayOfType, Bar ],
        destinations: [(
            output.ts_path(),
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

    output.write_jest(
        "ArrayOfType, Bar",
        r#"

describe('Struct with Array of Types Validation', ()=>{

  it('validates an object: `{ records: [{ val: true }, {val: false }] }` which conforms to ArrayOfType', ()=>{
    expect(() => {
        ArrayOfType.tryValidate({ records: [{ val: true }, {val: false }] });
    }).not.toThrow();
  });

  it('validates an empty array: `{ records: [] }` which conforms to ArrayOfType', ()=>{
    expect(() => {
        ArrayOfType.tryValidate({ records: [] });
    }).not.toThrow();
  });

  it('throws an error validating an object: `{ records: [{ val: true }, {val: false }, [3]] }` which has one value not conforming to the type', ()=>{
    expect(() => {
        ArrayOfType.tryValidate({ records: [{ val: true }, {val: false }, [3]] })
    }).toThrow();
  });

})
    "#,
    )?;

    output.run_ts()
}
