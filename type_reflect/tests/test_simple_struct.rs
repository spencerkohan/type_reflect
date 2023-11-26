mod common;

use anyhow::Result;
use common::*;

use serde::{Deserialize, Serialize};
use type_reflect::*;

#[derive(Reflect, Serialize, Deserialize)]
pub struct Foo {
    pub x: f32,
}

pub const SCOPE: &'static str = "test_simple_struct";

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

describe('Simple Struct Validation', ()=>{
  it("validates an object: `{x: 7}` matching the type `Foo` without throwing", ()=>{
    expect(() => {
        Foo.validate({x: 7})
    }).not.toThrow();
  });

  it("validates an object: `{x: -7}` matching the type `Foo` without throwing", ()=>{
    expect(() => {
        Foo.validate({x: -7})
    }).not.toThrow();
  });

  it("validates an object: `{x: 0}` matching the type `Foo` without throwing", ()=>{
    expect(() => {
        Foo.validate({x: 0})
    }).not.toThrow();
  });

  it("throws an error validating an object: `{x: '7'}` not matching the type `Foo`", ()=>{
    expect(() => {
        Foo.validate({x: '7'})
    }).toThrow();
  });

  it("throws an error validating an object: `{y: 7}` not matching the type `Foo`", ()=>{
    expect(() => {
        Foo.validate({y: 7})
    }).toThrow();
  });

  it("throws an error validating a string: `foo` not matching the type `Foo`", ()=>{
    expect(() => {
        Foo.validate('foo')
    }).toThrow();
  });

  it("throws an error validating a number: 7 not matching the type `Foo`", ()=>{
    expect(() => {
        Foo.validate(7)
    }).toThrow();
  });

  it("throws an error validating a boolean: false not matching the type `Foo`", ()=>{
    expect(() => {
        Foo.validate(false)
    }).toThrow();
  });
})
    "#,
    )?;

    output.run_ts()
}

#[test]
fn test_parsing() -> Result<()> {
    let output = init_path(SCOPE, "test_parsing");

    export_types!(
        types: [ Foo ],
        destinations: [(
            output.ts_path(),
            // prefix: TESTING_PREFIX,
            // postfix: post,
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

describe('Simple Struct Parsing', ()=>{
  it('parses json: `{"x": 7}` matching the type `Foo` without throwing', ()=>{
    expect(() => {
        const foo = Foo.parse(`{"x": 7}`);
        expect(foo.x).toBe(7);
    }).not.toThrow();
  });
  it('parses json: `{"x": -7}` matching the type `Foo` without throwing', ()=>{
    expect(() => {
        const foo = Foo.parse(`{"x": -7}`);
        expect(foo.x).toBe(-7);
    }).not.toThrow();
  });
  it('parses json: `{"x": 0}` matching the type `Foo` without throwing', ()=>{
    expect(() => {
        const foo = Foo.parse(`{"x": 0}`);
        expect(foo.x).toBe(0);
    }).not.toThrow();
  });
  it('parses json: `{"x": 3.14159}` matching the type `Foo` without throwing', ()=>{
    expect(() => {
        const foo = Foo.parse(`{"x": 3.14159}`);
        expect(foo.x).toBe(3.14159);
    }).not.toThrow();
  });
  it('throws an error parsing a string: `{"y": 7}` not matching the type `Foo`', ()=>{
    expect(() => {
        Foo.parse(`{"y": 7}`)
    }).toThrow();
  });
  it('throws an error parsing a string: `qewcm9823d` not matching the type `Foo`', ()=>{
    expect(() => {
        Foo.parse(`qewcm9823d`)
    }).toThrow();
  });
  it('throws an error parsing an invalid json string: `{x: 7}` not matching the type `Foo`', ()=>{
    expect(() => {
        Foo.parse(`{x: 7}`)
    }).toThrow();
  });
})
    "#,
    )?;

    output.run_ts()
}
