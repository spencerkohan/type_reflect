# Changelog

All notable changes to the `type_reflect` workspace.

## 0.9.0 - 2026-08-17

Released alongside `type_reflect_core` 0.6.0 and `type_reflect_macros` 0.7.0
(`ts_quote` / `ts_quote_macros` unchanged).

### Added

- Full serde rename support across all emitters (`TypeScript`, `TSValidation`,
  `Zod`):
  - `#[serde(rename_all)]` on structs and enums (all 7 supported inflections),
    applied at container and field/variant level
  - `#[serde(rename)]` on individual fields and enum variants — explicit
    renames win over `rename_all` and are not themselves inflected
  - `#[serde(tag)]` and `#[serde(content)]` for internally tagged enums
  - `#[serde(untagged)]` for bare-content enums
- `EnumCase::serialized_name()` — single source of truth for the
  post-serialization name of a case; all emitters route through it
- Zod emitter support for externally tagged and untagged (bare-content)
  enums, including single-variant degenerate cases (a 1-member `z.union`
  is not valid TypeScript in zod v3, so a single case emits its bare
  schema / direct alias instead)
- Zod support for tuple structs: multi-field emits `z.tuple([...])`,
  single-field (newtype) emits the bare member schema — mirroring serde's
  array / bare-value serialization
- Quoting of non-identifier serialized names in emitted TypeScript:
  object keys and property accesses are quoted/bracket-accessed only when
  the name is not a valid identifier (e.g. kebab-case keys)
- `test_serde_rename` integration suite (22 tests): each case serializes
  with `serde_json` and the real output is validated against the emitted
  TSValidation `parse` and Zod schema via jest

### Changed

- Enum classification now follows serde's precedence: an explicit
  `#[serde(rename)]` wins over `rename_all`; `#[serde(untagged)]` wins
  over `tag`. Enums with a non-unit case classify as
  `Untagged` (bare content), `Complex` (with `tag`), or
  `ExternallyTagged` (default external tagging); unit-only enums stay
  `Simple` even under `untagged` (degenerate — serde emits a bare literal)
- `#[serde(...)]` attribute parsing is now lenient: unknown keys are
  skipped instead of aborting the derive, matching serde's behavior for
  keys this crate does not model
- Emitter output for renamed types changed to match serde's actual
  serialized form (previously several emitters emitted the Rust
  identifier instead of the serialized name — see Fixed)

### Fixed

- Zod emitter emitted case *identifiers* instead of serialized names for
  renamed variants
- Variant-level `#[serde(rename)]` was ignored by all emitters
- Externally tagged and untagged enums were unsupported (or misemitted)
  by the Zod emitter
- `#[serde(untagged)]` enums were emitted as tagged unions instead of
  bare-content unions
- Non-identifier serialized names (e.g. kebab-case field names) produced
  invalid TypeScript (unquoted object keys / property accesses)
- Single-variant enums emitted `z.union([...])` with one member, which
  fails to compile under TypeScript (zod v3 types `z.union` as a
  ≥2-element tuple)
- Derive no longer dies with `not yet implemented` panics at export time:
  - unit structs are rejected at derive time with a localized compile
    error
  - tuple variants in tagged enums without a `content` key are rejected
    at derive time with a localized compile error (serde rejects
    multi-tuples at compile time and panics at runtime for newtypes)
- 9 compile warnings removed (unused imports, dead-code in test/example
  fixtures); all crates now build warning-free

### Removed

- `#[ts]` attribute — dead and broken, zero usages
- `EnumType::Untagged` variant renamed to `EnumType::ExternallyTagged`
  (the old name is now taken by the serde `untagged` bare-content
  representation) — **breaking** for code matching on `EnumType`
- `to_ts_ident` (dead), `TypeExporter` trait ×2 (never implemented),
  `#[allow(unused)]` on the used `parse_serde_attrs`, and 12
  commented-out code blocks
