mod common;

use anyhow::Result;
use common::*;

use serde::{Deserialize, Serialize};
use type_reflect::*;

pub const SCOPE: &'static str = "test_ts_string";

struct Foo {}

#[test]
fn test_ts_str() -> Result<()> {
    let output = init_path(SCOPE, "test_ts_string");

    let prefix = ts_string! {
        const x = 4;
    };

    export_types!(
        types: [ ],
        destinations: [(
            output.ts_path(),
            prefix: prefix,
            emitters: [
                TSFormat(
                    tab_size: 2,
                    line_width: 80,
                ),
            ],
        )]
    )?;

    Ok(())
}

#[test]
fn test_with_str() -> Result<()> {
    let output = init_path(SCOPE, "test_with_str");

    let prefix = ts_string! {
        const x = "Foo";
    };

    export_types!(
        types: [ ],
        destinations: [(
            output.ts_path(),
            prefix: prefix,
            emitters: [
                TSFormat(
                    tab_size: 2,
                    line_width: 80,
                ),
            ],
        )]
    )?;

    Ok(())
}

#[test]
fn test_groups() -> Result<()> {
    let output = init_path(SCOPE, "test_groups");

    let prefix = ts_string! {
        const double = (x: number): number => {
            return x * 2;
        }
    };

    export_types!(
        types: [ ],
        destinations: [(
            output.ts_path(),
            prefix: prefix,
            emitters: [
                TSFormat(
                    tab_size: 2,
                    line_width: 80,
                ),
            ],
        )]
    )?;

    Ok(())
}

#[test]
fn test_ident_substitution() -> Result<()> {
    let output = init_path(SCOPE, "test_ident_substitution");

    let hola = 7;
    let foo = 3;
    let bar = 4;

    let prefix = ts_string! {
        const val = #hola + #{foo + bar};
        const lemon = #"`egg salad sandwich ${val}`";
        const peas = #"`egg salad sandwich ${val} == #foo`";
        const soup = #"`egg salad sandwich ${val} == #{foo - bar} something something`";
    };

    export_types!(
        types: [],
        destinations: [(
            output.ts_path(),
            prefix: prefix,
            emitters: [
                TSFormat(
                    tab_size: 2,
                    line_width: 80,
                ),
            ],
        )]
    )?;
    Ok(())
}
