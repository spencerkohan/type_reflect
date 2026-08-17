# Changelog

All notable changes to the `type_reflect` workspace.

## 0.10.0 - 2026-08-17

Released alongside `type_reflect_core` 0.7.0 and `type_reflect_macros`
0.8.0 (`ts_quote` / `ts_quote_macros` unchanged).

### Added

- Built-in type support for `std::path::Path`, `std::path::PathBuf`, and
  `serde_json::Value` in field positions, accepted both as bare names
  (`Path`, `PathBuf`, `Value`) and fully-qualified (`std::path::PathBuf`,
  `::std::path::Path`, `serde_json::Value`) — both spellings reflect
  identically:
  - `Path`/`PathBuf` collapse to `Type::String` (serde's wire form is a
    string, matching the existing numeric collapsing): `string`
    (TypeScript), `z.string()` (Zod), `typeof` string check
    (TSValidation)
  - `serde_json::Value` is the new `Type::JsonValue` variant: `any`
    (TypeScript), `z.any()` (Zod), no runtime check (TSValidation)
  - Composites (`Option<...>`, `Vec<...>`, `HashMap<...>`, …) and enum
    variants work with both types for free
- "Supported Types" section in the README, including the caveat that
  `Value` is matched by bare name (a user-defined `Value` type will be
  misread; use the fully-qualified spelling for your own)
- Tests: unit tests asserting emitter output for the new `Type` arms; new
  `test_built_in_types` integration suite covering bare-vs-fully-qualified
  equivalence, Rust emitter output (qualified spellings re-emit verbatim
  and need no extra imports), and a jest validation round-trip (structs,
  `Box<Path>`, externally tagged enum)

### Changed

- **Breaking** (`type_reflect_core`): the `Type` enum gained a `JsonValue`
  variant. The enum is not `#[non_exhaustive]`, so downstream exhaustive
  matches on `Type` need a new arm
- The macro parser now accepts multi-segment paths for the fully-qualified
  built-ins above; all other multi-segment or `::`-prefixed paths remain
  unsupported (fail fast, as before)

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
