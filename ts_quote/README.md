# TS Quote

[![Crates.io](https://img.shields.io/crates/v/ts_quote.svg)](https://crates.io/crates/ts_quote)
[![Documentation](https://docs.rs/ts_quote/badge.svg)](https://docs.rs/ts_quote)

*This crate is part of a larger workspace, see the [monorepo README](https://github.com/spencerkohan/type_reflect) for more details*

This crate provides a few quasi-quote macros for generating TypeScript from inside Rust.

It is built upon the [`Deno`](https://deno.com) project, and is interoperable with Deno's TypeScript representation.

The API is and usage are heavily inspired by the [`quote` crate](https://crates.io/crates/quote).

## Example usage:

Generate a Typescript string using `ts_string!`:

```
let ts: String = ts_string! { const foo: number = 42; }
// the value of ts is "const foo: number = 42;"
```

### Embedding values from Rust:

It's also possible to embed runtime values from Rust.

This should feel familiar to anyone who has used `quote` to generate Rust code:

```
let name = "foo";
let value: u32 = 7;

let ts: String = ts_string! { const #name: number = #{value + 1}; }
// the value of ts is "const foo: number = 8;"
```

Values can be included from Rust by prefixing with `#`.

To include a simple value, the pattern `#<identifier>` will be replaced with the value of `<identifier>`.

### Literal strings:

Sometimes it's not posible to represent TypeScript syntax as a valid Rust TokenStream.

For instance, if we try to use `ts_string` like so it will fail:

```
let ts: String = ts_string! { const text = 'some text here'; }
let ts: String = ts_string! { const text = `some other text here`; }
```

This is becuase 'some text here' and `some other text here` are not valid Rust token streams, so they will cause a compiler error before the ts_string proc macro can parse them.

To solve this problem, thie macros in this crate allow us to insert string literals directly into the output.

So for instance we can escape the examples above like so:

```
let ts: String = ts_string! { const text = #"'some text here'"; }
println!("{}", ts);
let ts: String = ts_string! { const text = #"`some other text here`:"; }
println!("{}", ts);
```

This will print:

```
const text = 'some text here';
const text = `some other text here`;
```

Substitutions are also supported inside literal strings, and raw strings can be literal strings:

```
let t = "text"
let here = "here
let ts: String = ts_string! { const text = r##"'some #t #{here}'"##; }
println!("{}", ts); // prints: const text = 'some text here';
```

## Deno Iterop

For interoperability with Deno, this library also provides the `to_quote!` macro.  This allows for creation of a `deno_ast::ParsedSource` object:

```
let ts: Result<ParsedSource, deno_ast::Diagnostic> = ts_quote! { const foo = truel; };
```

This crate also provides the `TSSource` convenience trait, which is implemented for `ParsedSource` (aliased as `TS`).

This trait provides a method for formatting:

```
let ts: ParsedSource = ts_quote! { const foo = truel; };
let source: anyhow::Result<Option<String>> = ts.formatted(None);
```

This method optionally takes a `dprint_plugin_typescript::configuration::Configuration` to control the output configuration.

If None is provided, a common sense default will be used for formatting.

Here's an example using a custom config:

```
let ts: ParsedSource = ts_quote! { const foo = truel; };
let config = ConfigurationBuilder::new()
    .indent_width(4)
    .line_width(80)
    .prefer_hanging(true)
    .prefer_single_line(false)
    .quote_style(QuoteStyle::PreferDouble)
    .next_control_flow_position(NextControlFlowPosition::SameLine)
    .build();
let source: anyhow::Result<Option<String>> = ts.formatted(Some(config));
```

The above example desugars to the following:

```
let ts: ParsedSource = ParsedSource::from_source( "const foo = truel;".to_string() );
let config = ConfigurationBuilder::new()
    .indent_width(4)
    .line_width(80)
    .prefer_hanging(true)
    .prefer_single_line(false)
    .quote_style(QuoteStyle::PreferDouble)
    .next_control_flow_position(NextControlFlowPosition::SameLine)
    .build();
let source: anyhow::Result<Option<String>> = ts.formatted(Some(config));
```
