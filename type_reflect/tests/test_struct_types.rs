mod common;

use anyhow::Result;
use common::*;

use serde::{Deserialize, Serialize};
use serde_json;
use type_reflect::*;

#[derive(Reflect, Serialize, Deserialize)]
pub struct Named {
    pub x: u32,
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct Tuple(u32);

#[derive(Reflect, Serialize, Deserialize)]
pub struct MultiTuple(u32, Named, Tuple);

pub const SCOPE: &'static str = "test_struct_types";

#[test]
fn test_named() -> Result<()> {
    let output = init_path(SCOPE, "test_named");

    let named = Named { x: 42 };
    let serialized = serde_json::to_string_pretty(&named)?;
    eprintln!("Serialzied named: {}", serialized);

    export_types!(
        types: [ Named ],
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
        "Named",
        ts_string! {
            describe("Named Validation", ()=>{
                it("Validates a valid named striuct", ()=>{
                    expect(() => {
                        Named.validate({x:42})
                    }).not.toThrow();
                });
            });
        }
        .as_str(),
    )?;

    output.run_ts()
}

#[test]
fn test_tuple() -> Result<()> {
    let output = init_path(SCOPE, "test_tuple");

    let tuple = Tuple(42);
    let serialized = serde_json::to_string_pretty(&tuple)?;
    eprintln!("Serialzied tuple: {}", serialized);

    export_types!(
        types: [ Tuple ],
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
        "Tuple",
        ts_string! {
            describe("Tuple Validation", ()=>{
                it("Validates a valid tuple", ()=>{
                    expect(() => {
                        Tuple.validate(42)
                    }).not.toThrow();
                });
            });
        }
        .as_str(),
    )?;

    output.run_ts()
}

#[test]
fn test_multi_tuple() -> Result<()> {
    let output = init_path(SCOPE, "test_multi_tuple");

    let tuple = MultiTuple(42, Named { x: 42 }, Tuple(7));
    let serialized = serde_json::to_string_pretty(&tuple)?;
    eprintln!("Serialzied tuple: {}", serialized);

    export_types!(
        types: [ Named, Tuple, MultiTuple ],
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
        "Named, Tuple, MultiTuple",
        ts_string! {
            describe("MultiTuple Validation", ()=>{
                it("Validates a valid multi-tuple", ()=>{
                    expect(() => {
                        MultiTuple.validate([
                            42,
                            { x: 7 },
                            99
                        ])
                    }).not.toThrow();
                });
            });
        }
        .as_str(),
    )?;

    output.run_ts()
}
