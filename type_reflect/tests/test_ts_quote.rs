mod common;

use anyhow::Result;
use common::*;

use ts_quote::ts_quote;
use ts_quote::{TSSource, TS};
use type_reflect::*;

pub const SCOPE: &'static str = "test_ts_quote";

#[test]
fn test_ident_substitution() -> Result<()> {
    let output = init_path(SCOPE, "test_ident_substitution");

    let hola = 7;
    let foo = 3;
    let bar = 4;

    let ts: TS = ts_quote! {
        const val = #hola + #{foo + bar};
        const lemon = #"`egg salad sandwich ${val}`";
        const peas = #"`egg salad sandwich ${val} == #foo`";
        const soup = #"`egg salad sandwich ${val} == #{foo - bar} something something`";
    }?;

    let prefix = ts.formatted(None)?.unwrap();

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
