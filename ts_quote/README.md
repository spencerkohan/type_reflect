# TS Quote

This crate provides a number of macros for generating TypeScript from inside Rust.

It is built upon the [`Deno`](https://deno.com) project, and is interoperable with Deno's Typescript representation.

## Example usage:

Generate a Typescript string using `ts_string!`:

```
let ts: String = ts_string! { const foo: number = 42; }
// the value of ts is "const foo: number = 42;"
```

Embed values from Rust:

```
let name = "foo";
let value: u32 = 7;

let ts: String = ts_string! { const #name: number = #{value + 1}; }
// the value of ts is "const foo: number = 8;"
```

Values can be included from Rust by prefixing with `#`.

To include a simple value, the pattern `#<identifier>` will be replaced with the value of `<identifier>`.
