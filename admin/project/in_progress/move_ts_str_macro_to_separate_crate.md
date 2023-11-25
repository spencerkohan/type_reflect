# Move ts_str macro to it's own crate

Currently I have ts_str in a general macros crate for supporting type_reflect.  It makes more sense for it to be in it's own crate.

## TODO:

- [x] Move `ts_str!` to the new crate
- [ ] Rename `ts_str!` to `ts_string!` and test
- [ ] Implement `ts_str!`
- [ ] Implement `ts_quote!`
