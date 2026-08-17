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

### G7. Kebab-case case names in untagged member cases emit invalid TypeScript

Unquoted object key in
`type_reflect/src/type_script/untagged_enum_type.rs::emit_member_case`
(`get-max?: …`) and unquoted property access in
`type_reflect/src/ts_validation/enum_type/untagged/union_case.rs`
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
- [ ] G2: parse variant-level `#[serde(rename)]` (store per-case rename on
      `EnumCase`) and apply it in all emitters
- [ ] G3: parse field-level `#[serde(rename)]` in `get_struct_member` (store
      per-field rename on `NamedField`) and apply it in all emitters
- [ ] G1: apply `T::inflection()` to case names/values in the Zod emitter
      (simple + complex)
- [ ] G4: implement (or explicitly reject with a compile error) untagged /
      default-external enums in the Zod emitter
- [ ] G6: parse `#[serde(untagged)]` and emit the bare-content representation
      (needs an `EnumType::SerdeUntagged` variant distinct from the current
      Untagged class, which models serde's default external representation)
- [ ] G7: quote non-identifier case names in untagged member-case keys /
      property access (`['get-max']`), in both the typescript and
      ts_validation emitters
- [ ] G8: guard single-variant complex enums in the Zod emitter
      (`z.union` needs ≥ 2 members)
- [ ] Flipping the red tests green is the definition of done; the 4 passing
      tests are the regression guard for already-covered cases
