mod common;

use anyhow::Result;
use common::*;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

use type_reflect::*;

// Bare spellings
#[derive(Reflect, Serialize, Deserialize)]
pub struct Bare {
    pub dir: PathBuf,
    pub data: Value,
    pub maybe: Option<Value>,
    pub list: Vec<Value>,
}

// Fully-qualified spellings — must reflect identically to `Bare`
#[derive(Reflect, Serialize, Deserialize)]
pub struct Qualified {
    pub dir: std::path::PathBuf,
    pub data: serde_json::Value,
    pub maybe: Option<serde_json::Value>,
    pub list: Vec<serde_json::Value>,
}

// `Path` is a DST (it wraps `OsStr`), so it can't be a by-value struct
// field; the usable owned form is `Box<Path>`. `Path` also implements
// Serialize but not Deserialize, so serde derives are limited to
// Serialize here.
#[derive(Reflect, Serialize)]
pub struct PathOnly {
    pub file: Box<Path>,
}

#[derive(Reflect, Serialize)]
pub struct QPathOnly {
    pub file: Box<::std::path::Path>,
}

#[derive(Reflect, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PathStatus {
    Missing,
    Present {
        path: PathBuf,
    },
    Raw(Value),
}

pub const SCOPE: &'static str = "test_built_in_types";

// Bare and fully-qualified spellings must produce the same reflection table.
#[test]
fn test_bare_and_fully_qualified_are_equivalent() {
    assert_eq!(
        format!("{:?}", Bare::fields()),
        format!("{:?}", Qualified::fields()),
        "bare and fully-qualified spellings must reflect identically"
    );
    assert_eq!(
        format!("{:?}", PathOnly::fields()),
        format!("{:?}", QPathOnly::fields()),
        "bare and fully-qualified spellings must reflect identically"
    );
}

// The Rust emitter re-emits the original source verbatim: qualified
// spellings need no extra imports in the generated file.
#[test]
fn test_rust_emitter() -> Result<()> {
    let output = init_path(SCOPE, "test_rust_emitter");
    let rs_path = output.ts_path().with_extension("rs");
    let _ = fs::remove_file(&rs_path);

    export_types!(
        types: [ Qualified, QPathOnly ],
        destinations: [Rust(rs_path.clone())]
    )?;

    let emitted = fs::read_to_string(&rs_path)?;
    for expected in [
        "use serde::{Deserialize, Serialize};",
        "use serde_json;",
        "#[derive(Debug, Clone, Serialize, Deserialize)]",
        "pub struct Qualified {",
        "pub dir: std::path::PathBuf,",
        "pub data: serde_json::Value,",
        "pub maybe: Option<serde_json::Value>,",
        "pub list: Vec<serde_json::Value>,",
        "pub struct QPathOnly {",
        "pub file: Box<::std::path::Path>,",
    ] {
        assert!(
            emitted.contains(expected),
            "missing {expected:?} in emitted rust:\n{emitted}"
        );
    }
    Ok(())
}

#[test]
fn test_validation() -> Result<()> {
    let output = init_path(SCOPE, "test_validation");

    export_types!(
        types: [ Bare, Qualified, PathOnly, QPathOnly, PathStatus ],
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
        "Bare, Qualified, PathOnly, QPathOnly, PathStatus",
        r#"
describe('Built-in types validation', () => {
  it('validates Bare with path strings and arbitrary JSON values', () => {
    expect(() => {
      Bare.validate({
        dir: '/tmp/foo',
        data: { nested: [1, 'two', true] },
        maybe: null,
        list: [1, 'two', true, null, { a: 1 }]
      })
    }).not.toThrow();
  });

  it('rejects a number for a PathBuf field', () => {
    expect(() => {
      Bare.validate({ dir: 42, data: null, maybe: null, list: [] })
    }).toThrow();
  });

  it('accepts any JSON for Value fields', () => {
    expect(() => {
      Bare.validate({ dir: '/tmp', data: 42, maybe: 'str', list: [42] })
    }).not.toThrow();
  });

  it('validates Qualified identically to Bare', () => {
    expect(() => {
      Qualified.validate({ dir: '/tmp', data: 42, maybe: null, list: [true] })
    }).not.toThrow();
  });

  it('rejects a number for a PathBuf field on Qualified', () => {
    expect(() => {
      Qualified.validate({ dir: 42, data: null, maybe: null, list: [] })
    }).toThrow();
  });

  it('validates PathOnly with a string', () => {
    expect(() => {
      PathOnly.validate({ file: 'x.txt' })
    }).not.toThrow();
  });

  it('rejects a number for a Box<Path> field', () => {
    expect(() => {
      PathOnly.validate({ file: 42 })
    }).toThrow();
  });

  it('validates QPathOnly identically to PathOnly', () => {
    expect(() => {
      QPathOnly.validate({ file: 'x.txt' })
    }).not.toThrow();
  });

  it('rejects a number for a Path field on QPathOnly', () => {
    expect(() => {
      QPathOnly.validate({ file: 42 })
    }).toThrow();
  });

  it('validates externally tagged PathStatus unit case', () => {
    expect(() => {
      PathStatus.validate('missing')
    }).not.toThrow();
  });

  it('validates externally tagged PathStatus struct case', () => {
    expect(() => {
      PathStatus.validate({ present: { path: '/x' } })
    }).not.toThrow();
  });

  it('rejects a number for the PathBuf in PathStatus struct case', () => {
    expect(() => {
      PathStatus.validate({ present: { path: 42 } })
    }).toThrow();
  });

  it('validates externally tagged PathStatus tuple case with any JSON value', () => {
    expect(() => {
      PathStatus.validate({ raw: 42 })
    }).not.toThrow();
  });
})
    "#,
    )?;

    output.run_ts()
}
