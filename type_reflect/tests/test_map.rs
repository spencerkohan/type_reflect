mod common;

use std::collections::HashMap;

use anyhow::Result;
use common::*;

use serde::{Deserialize, Serialize};
use type_reflect::*;

pub const SCOPE: &'static str = "test_map";

#[derive(Reflect, Serialize, Deserialize)]
pub struct Bar {
    val: bool,
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct MapOfPrimitive {
    records: HashMap<String, u32>,
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct MapOfType {
    records: HashMap<String, Bar>,
}

#[test]
fn test_array_of_primitive() -> Result<()> {
    let output = init_path(SCOPE, "test_map_of_primitive");

    export_types!(
        types: [ MapOfPrimitive ],
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
        "MapOfPrimitive",
        r#"

describe('Struct with Array of Primitives Validation', ()=>{

  it('validates an object: `{ records: {a: 42, b: 7, dog: 3, cat: 21} }` which conforms to MapOfPrimitive', ()=>{
    expect(() => {
        MapOfPrimitive.tryValidate({ records: {a: 42, b: 7, dog: 3, cat: 21} });
    }).not.toThrow();
  });

  it('validates an empty array: `{ records: {} }` which conforms to MapOfPrimitive', ()=>{
    expect(() => {
        MapOfPrimitive.tryValidate({ records: {} });
    }).not.toThrow();
  });

  it('throws an error validating an object: `{a: 42, b: 7, dog: "3", cat: 21}` which has one value not conforming to the type', ()=>{
    expect(() => {
        MapOfPrimitive.tryValidate({a: 42, b: 7, dog: "3", cat: 21})
    }).toThrow();
  });

})
    "#,
    )?;

    output.run_ts()
}

#[test]
fn test_nested_array() -> Result<()> {
    let output = init_path(SCOPE, "test_nested_map");

    export_types!(
        types: [ MapOfType, Bar ],
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
        "MapOfType, Bar",
        r#"

describe('Struct with Map of Types Validation', ()=>{

  it('validates an object: `{ records: {a: { val: true }, b: {val: false } } }` which conforms to MapOfType', ()=>{
    expect(() => {
        MapOfType.tryValidate({ records: {a: { val: true }, b: {val: false } } });
    }).not.toThrow();
  });

  it('validates an empty object: `{ records: {} }` which conforms to MapOfType', ()=>{
    expect(() => {
        MapOfType.tryValidate({ records: {} });
    }).not.toThrow();
  });

  it('throws an error validating an object: `{ records: {a: { val: true }, b: {val: false }, c: 32 } }` which has one value not conforming to the type', ()=>{
    expect(() => {
        MapOfType.tryValidate({ records: {a: { val: true }, b: {val: false }, c: 32 } })
    }).toThrow();
  });

})
    "#,
    )?;

    output.run_ts()
}
