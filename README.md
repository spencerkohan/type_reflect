# Type Reflect Monorepo

This is the monorepo for `type_reflect` and `ts_quote`.  It contains two main projects:

- [`type_reflect`](#1-type-reflect): Runtime reflection for Rust types, to facilitate easy bridging between Rust types and other languages
- [`ts_quote`](#1-ts-quote): Utilities for generating TypeScript code from Rust

## 1. 🪩 Type Reflect

`type_reflect` provides procedural macros to allow for runtime reflection on Rust types.  It's main goal is to facilitate sharing serializable types between languages, for example in the use-case of sharing types between a Rust webservice, consumed by a TypeScript client.

It provides some utilities out of the box for exporting Rust types to TypeScript, both as raw TS types and Zod schemas, and is designed to be extensible, so the user can implement custom type exporters to meet thier own specific use-case.

### 📦 Type Reflect Crates:

| Crate    | Description | Links    |
|----------|-------------|----------|
| → `type_reflect` ← | The main `type_reflect` crate for public consumption. | [![Github](https://img.shields.io/badge/github-source-blue?logo=github)](type_reflect) [![Crates.io](https://img.shields.io/crates/v/type_reflect.svg)](https://crates.io/crates/type_reflect) [![Documentation](https://docs.rs/type_reflect/badge.svg)](https://docs.rs/type_reflect) |
| `type_reflect_macros` | Procedural macro implementations for `type_reflect`.  This crate is for internal use, and the macros are re-exported by the `type_reflect` crate. | [![Github](https://img.shields.io/badge/github-source-blue?logo=github)](type_reflect_macros) [![Crates.io](https://img.shields.io/crates/v/type_reflect_macros.svg)](https://crates.io/crates/type_reflect_macros) [![Documentation](https://docs.rs/type_reflect_macros/badge.svg)](https://docs.rs/type_reflect_macros) |
| `type_reflect_core` | A crate for shared components used by both `type_reflect` and `type_reflect_macros`. | [![Github](https://img.shields.io/badge/github-source-blue?logo=github)](type_reflect_core) [![Crates.io](https://img.shields.io/crates/v/type_reflect_core.svg)](https://crates.io/crates/type_reflect_core) [![Documentation](https://docs.rs/type_reflect_core/badge.svg)](https://docs.rs/type_reflect_core) |

## 2. 🖊️ TS Quote

`ts_quote` provides procedural macros and utilities for generating TypeScript code in Rust.

Usage is similar to the popular [`quote` crate](https://crates.io/crates/quote) for Rust code generation.

### 📦 TS Quote Crates:

| Crate    | Description | Links    |
|----------|-------------|----------|
| → `ts_quote` ← | The main `ts_quote` crate for public consumption. | [![Github](https://img.shields.io/badge/github-source-blue?logo=github)](ts_quote) [![Crates.io](https://img.shields.io/crates/v/ts_quote.svg)](https://crates.io/crates/ts_quote) [![Documentation](https://docs.rs/ts_quote/badge.svg)](https://docs.rs/ts_quote) |
| `ts_quote_macros` | Procedural macro implementations for `ts_quote`.  This crate is for internal use, and the macros are re-exported by the `ts_quote` crate. | [![Github](https://img.shields.io/badge/github-source-blue?logo=github)](ts_quote_macros) [![Crates.io](https://img.shields.io/crates/v/ts_quote.svg)](https://crates.io/crates/ts_quote_macros) [![Documentation](https://docs.rs/ts_quote/badge.svg)](https://docs.rs/ts_quote_macros) |
