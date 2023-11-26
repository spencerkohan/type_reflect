# Type Reflect Monorepo

This is the monorepo for `type_reflect` and `ts_quote`.  It contains two main projects:

## 1. 🪩 Type Reflect

`type_reflect` provides procedural macros to allow for runtime reflection on Rust types.  It's main goal is to facilitate sharing serializable types between languages, for example in the use-case of sharing types between a Rust webservice, consumed by a TypeScript client.

It provides some utilities out of the box for exporting Rust types to TypeScript, both as raw TS types and Zod schemas, and is designed to be extensible, so the user can implement custom type exporters to meet thier own specific use-case.

### 📦 Type Reflect Crates:

| Crate    | Description | Links    |
|----------|-------------|----------|
| `type_reflect` | The main `type_reflect` crate for public consumption | [![Github](https://img.shields.io/badge/github-source-blue?logo=github)](/tree/main/type_reflect) [![Crates.io](https://img.shields.io/crates/v/type_reflect.svg)](https://crates.io/crates/type_reflect) [![Documentation](https://docs.rs/type_reflect/badge.svg)](https://docs.rs/type_reflect) |

## 2. 🖊️ TS Quote

`ts_quote` provides procedural macros and utilities for generating TypeScript code in Rust.

### 📦 TS Quote Crates:

| Crate    | Description | Links    |
|----------|-------------|----------|
| `ts_quote` | The main `ts_quote` crate for public consumption | [![Github](https://img.shields.io/badge/github-source-blue?logo=github)](/tree/main/ts_quote) [![Crates.io](https://img.shields.io/crates/v/ts_quote.svg)](https://crates.io/crates/ts_quote) [![Documentation](https://docs.rs/ts_quote/badge.svg)](https://docs.rs/ts_quote) |
