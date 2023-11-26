mod common;

use anyhow::Result;
use common::*;

use serde::{Deserialize, Serialize};
use type_reflect::*;

pub const SCOPE: &'static str = "test_nested";

#[derive(Reflect, Serialize, Deserialize)]
pub struct Bar {
    val: bool,
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct Foo {
    bar: Bar,
}

#[test]
fn test_validation() -> Result<()> {
    let output = init_path(SCOPE, "test_validation");

    export_types!(
        types: [ Foo, Bar ],
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
        "Foo, Bar",
        r#"

describe('Struct with Nested Type Validation', ()=>{

  it('validates an object: `{ bar: { val: true } }` which conforms to the nested types', ()=>{
    expect(() => {
        Foo.validate({ bar: { val: true } });
    }).not.toThrow();
  });

  it('throws an error validating an object: `{ bar: { val: "hola" } }` not conforming to the nested type', ()=>{
    expect(() => {
        Foo.validate({ bar: { val: "hola" } })
    }).toThrow();
  });

  it('throws an error validating an object: `{ bar: true }` not conforming to the nested type', ()=>{
    expect(() => {
        Foo.validate({ bar: true })
    }).toThrow();
  });

  it('throws an error validating an object: `{ baz: { val: true } }` not conforming to the outer type', ()=>{
    expect(() => {
        Foo.validate({ baz: { val: true } })
    }).toThrow();
  });

})
    "#,
    )?;

    output.run_ts()
}
