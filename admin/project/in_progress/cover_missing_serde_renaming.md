# Cover Missing Serde Renaming

The emitters (typescript, ts_validation, zod) don't cover all of the rename
attributes serde supports. Several cases produce emitted types that do not
match serde's actual serialization output.

This task was investigated with a new test suite:
`type_reflect/tests/test_serde_rename.rs`. Each test serializes a value with
`serde_json` (ground truth), emits the type through `TypeScript` +
`TSValidation` and `Zod` (separate output files — both families emit a
`Shape`/`Pet` symbol and would collide in one file), and asserts the emitted
validators accept the actual serde output. **4 pass / 13 fail — every failure
documents a gap below.** Tests are left red on purpose until fixed.

(`tests/output` gained a `zod@3` dev-dependency so Zod output can be parsed
in jest — before this, Zod output couldn't be executed at all.)

## Matrix (✓ pass / ✗ gap)

| Scenario (serde ground truth) | TS/TSValidation | Zod |
|---|---|---|
| `simple_rename_all` — enum `rename_all="camelCase"` (`"dogBreed"`) | ✓ | ✗ |
| `simple_case_rename` — variant `rename="custom_dog"` | ✗ | ✗ |
| `complex_external_rename_all` — `tag` + `rename_all="kebab-case"` | ✓ | ✗ |
| `complex_internal_rename_all` — `tag` + `rename_all="SCREAMING_SNAKE_CASE"` | ✓ | ✗ |
| `complex_external_case_rename` — variant `rename="renamed_circle"` | ✗ | ✗ |
| `complex_external_field_rename` — field `rename="custom_radius"` | ✗ | ✗ |
| `complex_external_variant_rename_all` — variant `rename_all` on fields | ✓ | ✓ |
| `complex_combo_rename_alls` — enum + variant `rename_all` combined | ✓ | ✗ |
| `struct_rename_all` — struct `rename_all` | ✓ | ✓ |
| `struct_field_rename` — field `rename` | ✗ | ✗ |
| `untagged_unit_rename_all` — unit cases, enum `rename_all` | ✓ | (n/a, see G4) |
| `untagged_unit_case_rename` — unit case `rename="read-all-items"` | ✗ | (n/a) |
| `untagged_struct_representation` — case keys + variant field renames | ✓ | (n/a) |
| `untagged_kebab_case_key` — kebab-case member case keys | ✗ (invalid TS) | (n/a) |
| `serde_untagged_attribute` — real `#[serde(untagged)]` | ✗ | (n/a) |
| `tag_and_rename_all_single_attr_classification` | ✗ (misclassified) | — |
| `zod_untagged_supported` | — | ✗ (panics) |

## Gaps

### G1. Zod ignores enum-level `rename_all` entirely

`type_reflect/src/zod/enum_type.rs`: `emit_simple_enum_type` and
`generate_cases_enum` use raw `case.name` as enum values;
`generate_union_type` takes the inflection as `_inflection` and discards it;
`T::inflection()` is never called.
E.g. zod expects `'DogBreed' | 'CatType'` while serde emits `"dogBreed"`.

Evidence: `simple_rename_all`, `complex_external_rename_all`,
`complex_internal_rename_all`, `complex_combo_rename_alls` (TS passes, Zod
fails).

### G2. Variant-level `#[serde(rename = "...")]` ignored everywhere

The macro never parses it (`type_reflect_macros/src/type_def/enum_def.rs`:
`rename` arms are commented out of `EnumAttr`/`SerdeEnumAttr`; `EnumCase` has
no per-case rename field — its `inflection` comes from the case's
`rename_all`, which serde applies to *fields*, not the case name).

Covers the untagged unit-case case: serde serializes a renamed unit case as
the custom string, all emitters emit the raw name.

Evidence: `simple_case_rename` (TS + Zod fail), `complex_external_case_rename`
(both fail), `untagged_unit_case_rename` (TS fails).

### G3. Field-level `#[serde(rename = "...")]` ignored everywhere

`type_reflect_macros/src/type_def/syn_type_utils.rs::get_struct_member`
doesn't read `field.attrs` at all. Affects struct fields and struct-variant
fields in all three emitters.

Evidence: `struct_field_rename` (TS + Zod fail),
`complex_external_field_rename` (both fail on the renamed field; unrenamed
fields in the same type pass).

### G4. Zod doesn't support the untagged / default-external enum class at all

`unimplemented!()` panic, `type_reflect/src/zod/enum_type.rs:17`.

Evidence: `zod_untagged_supported`.

### G5. Multi-key `#[serde]` attributes silently drop unknown keys

`impl_parse!` (`type_reflect_core/src/lib.rs`) errors on the first unknown
key → the *whole attribute* is rejected → `parse_serde_attrs`
(`type_reflect_macros/src/attribute_utils.rs`) swallows the error (the
warning is commented out).

Consequence: `#[serde(tag = "_case", rename_all = "kebab-case")]` on one line
loses the `tag` → the enum is misclassified as Untagged instead of Complex →
TS emits the wrong representation and Zod panics (G4). The test suite
works around it by splitting into two `#[serde]` attributes.

The same mechanism silently drops other real serde attributes containing
keys the macro doesn't know: `untagged` (→ G6), `alias`,
`deny_unknown_fields`, ….

Evidence: `tag_and_rename_all_single_attr_classification` (asserts
`EnumType::Complex`, gets Untagged).

### G6. `#[serde(untagged)]` is ignored → wrong representation

Real serde untagged serializes a struct case as bare content
(`{"my_radius":5}`, no case key) and a unit case as `null`. The library
emits the keyed default-external representation (`{"Circle": ...}` /
`"Null"` string literal) instead. Both forms are rejected by the emitted
validators.

Evidence: `serde_untagged_attribute`.

### G7. Kebab-case case names in externally-tagged member cases emit invalid TypeScript

Unquoted object key in
`type_reflect/src/type_script/externally_tagged_enum_type.rs::emit_member_case`
(`get-max?: …`) and unquoted property access in
`type_reflect/src/ts_validation/enum_type/externally_tagged/union_case.rs`
(`input.get-max`). The whole emitted file fails to compile.
Only kebab-case among the 7 serde inflection forms is affected; the rest
produce valid identifiers.

Evidence: `untagged_kebab_case_key` (suite-level compile failure).

### G8. (incidental, not rename) Zod emits `z.union([single])` for single-variant complex enums

Invalid under zod v3 (requires ≥ 2 members). Discovered because the harness
had to add a second variant to four single-variant complex scenarios to get
past it. `type_reflect/src/zod/enum_type.rs::generate_union_schema` has no
single-member guard.

## Confirmed covered (passing)

- Struct `rename_all` — `struct_rename_all` (TS + Zod).
- Enum `rename_all` for complex external and internal tagged case names
  (TS side) — `complex_external_rename_all`, `complex_internal_rename_all`.
- Variant-level `rename_all` on variant fields (TS + Zod) —
  `complex_external_variant_rename_all`.
- Enum + variant `rename_all` combo: case names + fields (TS side) —
  `complex_combo_rename_alls`.
- Unit-case `rename_all` and case-key + field renames in the
  untagged/default-external class — `untagged_unit_rename_all`,
  `untagged_struct_representation`.
- All 7 serde inflection spellings map correctly to `Inflection` variants;
  the mapping itself (`Inflection::try_from` / `apply`) is faithful.

## Minor notes

- The untagged/default-external model was verified against real
  `serde_json` output for ALL four variant shapes (live demo, all 4
  accepted by the emitted validators): unit -> `"A"`, single-tuple ->
  `{"B":1}`, multi-tuple -> `{"C":[1,2]}` (object keyed by case, array
  value — NOT `["C",1,2]`), struct -> `{"D":{"x":3}}`. The emitters' model
  (`'A' | { B?: T; C?: [A, B]; D?: { fields } }` + key-presence
  validation) matches serde exactly for every shape; no divergence. (An
  earlier draft of this note claimed a multi-element tuple divergence —
  that was a mis-memory of serde's encoding, retracted.) Multi-element
  tuple variants remain untested in the suite (no TODO, low risk).

- `parse_assign_inflection` maps unknown `rename_all` values to
  `Inflection::None` silently (typo → raw names, no error).
- Type-level `#[serde(rename)]` on the enum/struct itself is not applied to
  emitted type names. Not a serialization gap (serde never writes that name
  into JSON), but differs from ts-rs behavior if that matters.
- `EnumCase::name_with_inflection` is dead code.
- The TS and Zod emitters can't share one output file (duplicate
  `export enum Shape` etc.) — that's what forced the two-file test layout.

## Todo

- [x] G5: lenient serde parsing — `impl_parse!` now skips unknown keys
      (and their values: `key`, `key = "lit"`, `key(...)`) instead of
      rejecting the whole attribute. `#[serde(tag = "_case", rename_all =
      "kebab-case")]` in one attribute parses correctly (test
      `tag_and_rename_all_single_attr_classification` now passes). Design
      decision made along the way: the vestigial `#[ts]` attribute was
      removed entirely (unused anywhere, hard-error on any use of it on
      enums), so serde parsing is now the only consumer of `impl_parse!`.
      Skipped: compiler warning on skipped keys (not reachable from
      `type_reflect_core` without linking `proc_macro`).
- [x] G2: variant-level `#[serde(rename)]` parsed and stored on
      `EnumCase.rename`; new `EnumCase::serialized_name(enum_inflection)`
      accessor (rename wins, else enum-level `rename_all` inflected —
      serde's precedence) is now the single source for the case name
      wherever it appears in emitted output (TS simple/complex/untagged,
      TSValidation untagged, zod simple/complex enum values). Tests
      `simple_case_rename`, `complex_external_case_rename`,
      `untagged_unit_case_rename` pass. Note: `rename` is parsed via
      `RenameAllAttr` (also on types, where it is correctly ignored — type
      names are not part of serde's JSON output).
- [x] G3: field-level `#[serde(rename)]` parsed in `get_struct_member` via
      `RenameAllAttr::from_attrs(&field.attrs).rename`, stored on
      `NamedField.rename`; new `NamedField::serialized_name(inflection)`
      accessor (same rename-wins-over-`rename_all` rule as G2) is now the
      single source for field names in all emitters (TS `named_member`,
      TSValidation `named_field_validations`, zod `struct_member` + complex
      variant fields). Tests `struct_field_rename`,
      `complex_external_field_rename` pass (TS + zod).
- [x] G1: resolved as a byproduct of G2 — the zod simple/complex case
      values now go through `EnumCase::serialized_name()`, which applies the
      enum-level inflection. Tests `simple_rename_all`,
      `complex_external_rename_all`, `complex_internal_rename_all`,
      `complex_combo_rename_alls` pass (zod included).
- [x] G4: Zod untagged (default-external) enums implemented in
      `type_reflect/src/zod/enum_type.rs::emit_untagged_enum_type` — mirrors
      the TS/TSValidation untagged model: unit case -> `z.literal("Name")`,
      single-tuple -> `z.object({ "Name": <inner type> })`, multi-tuple ->
      `z.object({ "Name": z.tuple([...]) })`, struct ->
      `z.object({ "Name": z.object({...}) })` combined in `z.union`.
      Case keys are always quoted (`"CIRCLE"`), so kebab-case names are
      valid from the start (the TS-side kebab fix, G7, does not need to
      cover zod). The `zod_untagged_supported` test was upgraded from a
      no-panic assert to the suite's standard pattern (TS + zod both
      checked against real serde output).
- [x] G6: `#[serde(untagged)]` parsed and emitted as the bare-content
      representation. Design (user decision, option B): the old
      `EnumType::Untagged` class was renamed `EnumType::ExternallyTagged`
      (it models serde's DEFAULT external form: unit -> `"Name"`, others
      -> `{ "Name": content }`); the name `Untagged` now belongs to the new
      serde-untagged class (bare content: struct -> `{ fields }`,
      tuple -> `[a, b]`, unit -> `null`). Emitter modules/functions renamed
      to match (`type_script/externally_tagged_enum_type.rs`,
      `ts_validation/enum_type/externally_tagged/`, renamed functions in
      `zod/enum_type.rs`).
      - macro: `EnumAttr.untagged: bool` parsed by `impl_parse!` (bare key);
        classification: complex + untagged -> `Untagged` (wins over `tag` —
        serde errors on that combination, so unobservable in correct code),
        complex + tag -> `Complex`, complex + neither ->
        `ExternallyTagged`.
      - TS: `export type Shape = ShapeCaseCircle | null` — bare-content
        union (unit -> `null`), reusing the shared case-type machinery
        (multi-tuple/named cases keep their helper types).
      - TSValidation: first-match-wins try/catch chain over the case
        contents in declaration order (mirrors serde's untagged
        deserialization); per-case content validators reused from the
        externally_tagged module; unit -> `input === null`; single-tuple ->
        inline `type_validation`.
      - zod: union of bare content schemas (unit -> `z.null()`); a
        single-case enum emits the bare schema instead of an invalid
        1-member `z.union` (G8's guard, applied to the new emitter — the
        OLD emitters are G8's remaining scope).
      Test `serde_untagged_attribute` extended to also emit + check the Zod
      schema; now passes (TS + zod both accept `{"my_radius":5}` and
      `null`). Skipped: `#[serde(untagged)]` on an all-unit enum stays
      `Simple` (serde would serialize every variant to `null` — degenerate
      in serde itself; revisit if ever needed).
- [x] G7: quote non-identifier names wherever they are emitted as unquoted
      TS keys / property access. New runtime-crate helpers in
      `type_reflect/src/lib.rs`: `ts_key(name)` (object key: `Circle` or
      `"get-max"`) and `ts_access(prefix, name)` (`input.Circle` or
      `input["get-max"]`); valid identifiers pass through unchanged, so
      existing output is byte-identical. The old dead
      `raw_name_to_ts_field` in `type_reflect_macros/src/utils.rs` was
      deleted (a proc-macro crate can't be a normal dependency, so the
      logic lives in the runtime crate instead).
      Sites fixed:
      - TS object keys: type_script `named_member` (struct + variant
        fields), `emit_member_case` (externally-tagged case key), complex
        `generate_union_type` (tag + content keys); zod `struct_member`
        (fields), complex `generate_union_type` (tag + content keys).
        (Zod externally-tagged / serde-untagged case keys were already
        always quoted in G4/G6.)
      - TS property access: ts_validation `named_field_validations`
        (fields), externally-tagged `union_case` (case key), complex
        `case_type` (tag key + content-key prefixes).
      Added regression test `kebab_struct_field` (kebab-case field via
      struct `rename_all`, all three emitters) — the pre-existing
      `untagged_kebab_case_key` test only covered case keys. Both green.
      Tag/content-key sites use the same helpers but are not separately
      tested (would require a non-identifier `#[serde(tag = "...")]`).
      Skipped: TS reserved-word table in the identifier check — a rename
      to e.g. `if` still emits invalid dot access (marked with a
      `ponytail:` comment at the helper).
- [ ] G8: guard single-member unions in the Zod emitter (`z.union` needs
      ≥ 2 members) — covers single-variant complex enums AND single-case
      externally-tagged enums (e.g. `enum E { Only { x: u32 } }`), both of
      which currently emit `z.union([one])`. (The new serde-untagged
      emitter added in G6 already has the single-member guard.)
- [ ] Flipping the red tests green is the definition of done; the 4 passing
      tests are the regression guard for already-covered cases
