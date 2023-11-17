mod common;

use anyhow::Result;
use common::*;

use serde::{Deserialize, Serialize};
use type_reflect::*;

#[derive(Reflect, Serialize, Deserialize)]
pub enum Pet {
    Dog,
    Cat,
}

pub const SCOPE: &'static str = "test_simple_enum";

#[test]
fn test_validation() -> Result<()> {
    let output = init_path(SCOPE, "test_validation");

    export_types!(
        types: [ Pet ],
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
        "Pet",
        r#"

describe('Simple Enum Validation', ()=>{

  it("validates an object: `Dog`", ()=>{
    expect(() => {
        Pet.tryValidate(`Dog`)
    }).not.toThrow();
  });

  it("validates an object: `Cat`", ()=>{
    expect(() => {
        Pet.tryValidate(`Cat`)
    }).not.toThrow();
  });

  it("throws an error validating an number: `7`", ()=>{
    expect(() => {
        Pet.tryValidate(7)
    }).toThrow();
  });

  it("throws an error validating an object: `{tag: 'Dog'}`", ()=>{
    expect(() => {
        Pet.tryValidate({tag: 'Dog'})
    }).toThrow();
  });

})
    "#,
    )?;

    output.run_ts()
}
