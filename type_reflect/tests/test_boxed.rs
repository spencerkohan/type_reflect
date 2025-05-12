mod common;

use anyhow::Result;
use common::*;

use serde::{Deserialize, Serialize};
use ts_quote::*;
use type_reflect::*;

pub const SCOPE: &'static str = "test_boxed";

#[derive(Reflect, Serialize, Deserialize)]
pub struct Bar {
    val: bool,
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct BoxOfPrimitive {
    boxed: Box<u32>,
    // boxed: u32,
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct BoxOfType {
    boxed: Box<Bar>,
}

#[test]
fn test_box_of_primitive() -> Result<()> {
    let output = init_path(SCOPE, "test_box_of_primitive");

    export_types!(
        types: [ BoxOfPrimitive ],
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
        "BoxOfPrimitive",
        r#"

    describe('Box of Primitive Validation', ()=>{

      it('validates an object: `{ boxed: 42 }` which conforms to BoxOfPrimitive', ()=>{
        expect(() => {
            BoxOfPrimitive.validate({ boxed: 42 });
        }).not.toThrow();
      });

      it('throws an error validating a malformed object: `{ boxed: [42] }`', ()=>{
        expect(() => {
            BoxOfPrimitive.validate({ boxed: [42] })
        }).toThrow();
      });

    })
        "#,
    )?;

    output.run_ts()
}

#[test]
fn test_box_of_type() -> Result<()> {
    let output = init_path(SCOPE, "test_box_of_type");

    export_types!(
        types: [ Bar, BoxOfType ],
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
        "Bar, BoxOfType",
        r#"

    describe('Box of Type Validation', ()=>{

      it('validates an object: `{ boxed: { val: true } }` which conforms to BoxOfType', ()=>{
        expect(() => {
            BoxOfType.validate({ boxed: { val: true } });
        }).not.toThrow();
      });

      it('throws an error validating a malformed object: `{ boxed: [true] }`', ()=>{
        expect(() => {
            BoxOfType.validate({ boxed: [true] })
        }).toThrow();
      });

    })
        "#,
    )?;

    output.run_ts()
}

#[derive(Reflect, Serialize, Deserialize)]
#[serde(tag = "_case", content = "data")]
pub enum Tree {
    Leaf(bool),
    Subtree { left: Box<Tree>, right: Box<Tree> },
}

#[test]
fn test_nested_box() -> Result<()> {
    let output = init_path(SCOPE, "test_nested_box");

    export_types!(
        types: [ Tree ],
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
        "Tree, TreeCase, TreeCaseKey",
        ts_quote! {

            describe("Nested Box Validation", ()=>{

              it("validates an object: `{ ... }` which conforms to Tree", ()=>{
                expect(() => {
                    Tree.validate(
                        {
                            _case: TreeCaseKey.Subtree,
                            data: {
                                left: { _case: TreeCaseKey.Leaf, data: true },
                                right: {
                                    _case: TreeCaseKey.Subtree,
                                    data: {
                                        left: { _case: TreeCaseKey.Leaf, data: true },
                                        right: { _case: TreeCaseKey.Leaf, data: false }
                                    }
                                }
                            }
                        }
                    );
                }).not.toThrow();
              });

              it("throws an error validating a malformed object: `{ boxed: [true] }`", ()=>{
                expect(() => {
                    Tree.validate({ boxed: [true] })
                }).toThrow();
              });

            })


        }?
        .formatted(None)?
        .as_str(),
    )?;

    output.run_ts()
}
