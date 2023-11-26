# Move ts_str macro to it's own crate

Currently I have ts_str in a general macros crate for supporting type_reflect.  It makes more sense for it to be in it's own crate.

## TODO:

- [x] Move `ts_str!` to the new crate
- [x] Rename `ts_str!` to `ts_string!` and test
- [x] Implement `ts_quote!`
    - [x] Implement TS type
    - [x] Implement `ts_quote` macro
    - [x] Document
