mod common;

use anyhow::Result;
use common::*;

use serde::{Deserialize, Serialize};
use type_reflect::*;

#[derive(Reflect, Serialize, Deserialize)]
pub struct Foo {
    pub x: f32,
    pub y: Option<f32>,
}

pub const SCOPE: &'static str = "test_optional";

#[test]
fn test_validation() -> Result<()> {
    let output = init_path(SCOPE, "test_validation");

    export_types!(
        types: [ Foo ],
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
        "Foo",
        r#"

describe('Struct with Optional Member Validation', ()=>{

  it("validates an object: `{x: 7, y: 42}` with both the requred and optional members", ()=>{
    expect(() => {
        Foo.validate({x: 7, y: 42})
    }).not.toThrow();
  });

  it("validates an object: `{x: 7}` matching the type `Foo` without the optional member `y`", ()=>{
    expect(() => {
        Foo.validate({x: 7})
    }).not.toThrow();
  });

  it("throws an error validating an object: `{y: 42}` missing the required member `x`", ()=>{
    expect(() => {
        Foo.validate({y: 42})
    }).toThrow();
  });

})
    "#,
    )?;

    output.run_ts()
}
