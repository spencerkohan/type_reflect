mod common;

use anyhow::Result;
use common::*;

use serde::{Deserialize, Serialize};
use type_reflect::*;

#[derive(Reflect, Serialize, Deserialize)]
pub struct Rectangle {
    width: f32,
    height: f32,
}

#[derive(Reflect, Serialize, Deserialize)]
#[serde(tag = "_case", content = "data")]
pub enum Shape {
    Circle { radius: f32 },
    Square { side: f32 },
    Rectangle(Rectangle),
    ScaledRectangle(Rectangle, u32),
    Null,
}

pub const SCOPE: &'static str = "test_adt";

#[test]
fn test_validation() -> Result<()> {
    let output = init_path(SCOPE, "test_validation");

    export_types!(
        types: [ Shape, Rectangle ],
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
        "Shape, Rectangle, ShapeCase",
        ts_string! {
            describe("ADT Validation", ()=>{
                it("Validates a Null variant: {_case: ShapeCase.Null}", ()=>{
                    expect(() => {
                        Shape.tryValidate({
                            _case: ShapeCase.Null
                        })
                    }).not.toThrow();
                });
                it("Validates a Circle variant: {_case: ShapeCase.Circle, data: { radius: 1.7} }", ()=>{
                    expect(() => {
                        Shape.tryValidate({
                            _case: ShapeCase.Circle,
                            data: {
                                radius: 1.7
                            }
                        })
                    }).not.toThrow();
                });
                it("Validates a Rectangle variant: {_case: ShapeCase.Rectangle, data: { width: 1, height: 2} }", ()=>{
                    expect(() => {
                        Shape.tryValidate({
                            _case: ShapeCase.Rectangle,
                            data: {
                                width: 1,
                                height: 2
                            }
                        })
                    }).not.toThrow();
                });
                it("Validates a ScaledRectangle variant: {_case: ShapeCase.ScaledRectangle, data: [{ width: 1, height: 2}, 0.5] }", ()=>{
                    expect(() => {
                        Shape.tryValidate({
                            _case: ShapeCase.ScaledRectangle,
                            data: [
                                {
                                    width: 1,
                                    height: 2
                                },
                                0.5
                            ]
                        })
                    }).not.toThrow();
                });
                it("Doesn't Validate an incorrect ScaledRectangle variant: {_case: ShapeCase.Circle, data: [{ width: 1, height: 2}, 0.5] }", ()=>{
                    expect(() => {
                        Shape.tryValidate({
                            _case: ShapeCase.Circle,
                            data: [
                                {
                                    width: 1,
                                    height: 2
                                },
                                0.5
                            ]
                        })
                    }).toThrow();
                });

            });
        }
        .as_str(),
    );

    output.run_ts()
}
