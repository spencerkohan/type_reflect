# Extend Built-In Type Support

*As a developer, I want `Path`/`PathBuf` and `serde_json::Value` fields accepted by the `Reflect` macro and emitted correctly by all emitters, so my exported types match what Serde actually serializes.*

## Background

Field types are resolved in exactly one place: `SynTypeBridge::to_type` →
`simple_type` in `type_reflect_macros/src/type_def/syn_type_utils.rs`.
Unrecognized names become `Type::Named` and are only valid if the type is
itself `Reflect`-declared — so unknown built-ins either fail to compile at
the destination or emit a dangling schema reference.

Note: `to_type` currently only accepts single-segment paths without a
leading `::` (guard: `leading_colon.is_none() && segments.len() == 1`), so
fully-qualified spellings like `std::path::PathBuf` fail with
"Unsupported type". Both the bare and fully-qualified spellings must be
accepted and produce the same `Type`.

`Type` (in `type_reflect_core/src/type_description.rs`) is matched
exhaustively in four places. Any new variant must be handled in all of them:

1. `type_reflect_macros/src/type_def/type_utils.rs` — `TypeBridge::emit_type`
2. `type_reflect/src/type_script/mod.rs` — `to_ts_type`
3. `type_reflect/src/zod/mod.rs` — `to_zod_type`
4. `type_reflect/src/ts_validation/validation/type_validation.rs` — `type_validation`

The `Rust` emitter re-emits the original source tokens verbatim, so it never
needs changes for new built-ins.

Type aliases route through the same pipeline (`type_alias_def.rs` uses
`to_type()`), so `type P = PathBuf;` works for free once the field type does.
Composites (`Option<T>`, `Vec<T>`, `HashMap<K, V>`) recurse via `to_type()`,
so they work for free as well.

---

## Proposal 1: `Path` and `PathBuf` → collapse to `Type::String`

Serde serializes both `Path` and `PathBuf` to a JSON string. The wire
representation *is* a string, and that is what this crate models — the same
collapsing already done for `u8..u64` → `Type::UnsignedInt`,
`i8..i64` → `Type::Int`, `f32`/`f64` → `Type::Float`.

**Parse changes** in `syn_type_utils.rs`:

Bare names — one line in `simple_type`:

```rust
"Path" | "PathBuf" => Type::String,
```

Fully-qualified names — relax the `to_type` guard to any `syn::Type::Path`
without `qself`, and match fully-qualified built-ins by joined segment
name (joining drops the leading `::`, so `std::path::Path` and
`::std::path::Path` normalize identically):

```rust
let name = type_path.path.segments
    .iter().map(|s| s.ident.to_string()).collect::<Vec<_>>().join("::");

// fully-qualified built-ins (checked when segments.len() > 1)
"std::path::Path" | "std::path::PathBuf" => Ok(Type::String),
"serde_json::Value" => Ok(Type::JsonValue), // added with Proposal 2
```

Single-segment behavior — including the existing rejection of `::Foo` — is
unchanged. No generics handling needed: fully-qualified built-ins take none,
and `Option<std::path::PathBuf>` recurses into `to_type()` for the inner
type, hitting the same arm.

Resulting output, all correct against the serde wire format:

| Emitter        | Output        |
|----------------|---------------|
| TypeScript     | `string`      |
| Zod            | `z.string()`  |
| TSValidation   | `typeof === 'string'` check |
| Rust           | unchanged (pass-through) |

No core, macro-emission, or emitter changes. `Option<PathBuf>`,
`Vec<Path>`, etc. work automatically.

**Alternative (not recommended now):** a dedicated `Type::Path` variant,
emitted as `string`/`z.string()` today. Only worth it if an emitter later
needs path-specific handling (e.g. a path-shaped `z.string()` regex).
Parsing is centralized, so adding it later is cheap. `Path`/`PathBuf` are
distinctive names, so the bare-name match is low-risk (same convention as the
existing `Option`/`Vec`/`HashMap` matches).

---

## Proposal 2: `serde_json::Value` → new `Type::JsonValue` variant

`Value` can hold *any* JSON value (`string | number | boolean | null |
object | array`). No existing `Type` variant expresses that, so a new one is
required.

**Core** — `type_reflect_core/src/type_description.rs`:

```rust
pub enum Type {
    // ...
    /// Any JSON value; source type `serde_json::Value`
    JsonValue,
}
```

**Parse** — `simple_type` in `syn_type_utils.rs`:

```rust
"Value" => Type::JsonValue,
```

plus the `serde_json::Value` arm in the fully-qualified table from
Proposal 1 — so `data: Value`, `data: serde_json::Value`, and `data: ::serde_json::Value` all work.

*Note:* this matches the bare name `Value`, the same convention as the
existing `Option`/`Vec`/`HashMap` matches. A proc macro cannot see the
surrounding imports, so a user-defined `Value` type in scope would be
misread. Accepted risk, documented in the crate README alongside the
existing caveats. (An opt-in attribute would be the fix if it ever bites.)

**Macro emission** — `emit_type` in `type_utils.rs`:

```rust
Type::JsonValue => quote! { Type::JsonValue },
```

**Emitters** — one arm each:

| Emitter      | Arm                                    | Rationale |
|--------------|----------------------------------------|-----------|
| `to_ts_type` (type_script) | `Type::JsonValue => "any".to_string()` | honest; a precise recursive JSON union would require emitting a shared alias into every file — not worth it yet |
| `to_zod_type` (zod)        | `Type::JsonValue => "z.any()"`         | matches `any` |
| `type_validation` (ts_validation) | `Type::JsonValue => String::new()` | any JSON value passes; nothing to check |

Resulting output:

```ts
// struct Foo { data: Value, maybe: Option<Value>, list: Vec<Value> }
interface Foo {
  data: any;
  maybe: any;
  list: Array<any>;
}
```
```ts
// zod
data: z.any(),
maybe: z.any().optional(),
list: z.array(z.any()),
```

---

## Testing

Follow the existing `tests/test_struct_types.rs` pattern (new
`test_built_in_types.rs`, jest-based): a struct combining `Path`, `PathBuf`,
`Value`, `Option<Value>`, `Vec<Value>`, exported through
`TypeScript` + `TSValidation` + `TSFormat`, with jest cases asserting
validation accepts a path string / any JSON value and rejects a number for
the path fields.

Add a second struct using only the fully-qualified spellings
(`std::path::Path`, `::std::path::PathBuf`, `serde_json::Value`) and assert
its emitted output is identical to the bare-name struct's — pins both
spellings against drift.

Additionally, `to_ts_type`, `to_zod_type`, and `type_validation` are all
`pub`, so a small plain-Rust unit test (no jest) can assert the emitted
strings for the new arms directly — cheap guard against regressions.

## Acceptance Criteria

- [x] `#[derive(Reflect)]` structs/enum variants with `Path` and
      `PathBuf` fields compile and emit `string` (TS), `z.string()` (Zod),
      string check (TSValidation)
- [x] Fully-qualified spellings `std::path::Path`, `std::path::PathBuf`,
      `serde_json::Value` (with or without a leading `::`) are accepted and
      emit identically to their bare-name forms, including nested in
      composites (`Option<std::path::PathBuf>`, `Vec<serde_json::Value>`, …)
- [x] Structs/enum variants with `serde_json::Value` fields compile
      and emit `any` (TS), `z.any()` (Zod), no check (TSValidation)
- [x] Composites: `Option<...>`, `Vec<...>`, `HashMap<String, ...>` of both
      types emit correctly
- [x] Unit test asserting emitted strings for the new `Type` arms
- [x] Jest test round-trip (export + validate) for a struct using both types
- [x] README "supported types" updated; bare-`Value` caveat documented
- [x] Version bumps: adding a `Type` variant is breaking for
      `type_reflect_core` (enum is not `#[non_exhaustive]`) —
      core 0.6.0 → 0.7.0, macros 0.7.0 → 0.8.0, type_reflect 0.9.0 → 0.10.0

## Implementation Notes (2026-08-17)

- **Aliases are out of scope (deferred):** `#[derive(Reflect)]` on a type
  alias is rejected by rustc itself (E0774: derive may only be applied to
  structs, enums and unions), so the macro's `Item::Type` arm in
  `type_reflect_macros/src/lib.rs` is unreachable dead code. Alias support
  needs a different declaration mechanism (e.g. a function-like
  `reflect!` macro) — separate task.
- **`Path` is a DST:** `std::path::Path` wraps `OsStr`, so it cannot be a
  by-value struct field (such a struct is itself unsized and the generated
  `emit_struct::<Self>()` call requires `Sized`). `Path` is accepted by the
  parser (e.g. as `Box<Path>`) and the test covers it through
  `Box<Path>`; `PathBuf` is the normal by-value form.
- **Rust emitter test** asserts the generated file contains the
  re-emitted qualified spellings (`std::path::PathBuf`,
  `Box<::std::path::Path>`) — rustfmt preserves the leading `::`.

## Known Limitations

- **Rust emitter:** generated files re-emit original source, so a field typed
  as bare `PathBuf` appears as bare `PathBuf`, and the emitter's static
  `prefix()` does not include `use std::path::PathBuf;`. Exporting such a
  type via the Rust emitter requires the user to add the import through the
  destination `prefix:` arg (the macro cannot see sibling imports to do it
  automatically). Writing the field as `std::path::PathBuf` / `serde_json::Value`
  in the source sidesteps this entirely, since the qualified form re-emits
  verbatim and needs no import.
