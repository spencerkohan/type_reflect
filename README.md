# Type Reflect

[![Crates.io](https://img.shields.io/crates/v/type_reflect.svg)](https://crates.io/crates/type_reflect)
[![Documentation](https://docs.rs/type_reflect/badge.svg)](https://docs.rs/type_reflect)

***This project is currently fairly rough around the edges.  It's already usable in it's current state, but for instance the rust docs are WIP.  You may also expect to run into bugs and limitations, since so far it has only been tested on my own use-cases.  If you run into something, please report it so I can make this a better tool***

This library is implemented to make it easier to bridge between Rust types and Zod schemas for TypeScript.

This can be useful, for instance, in the context of a Rust Back-end service interacting with a TypeScript front-end.

Other solutions exist to solve similar problems, for instance the excellent [ts-rs crate](https://crates.io/crates/ts-rs), which I borrow from heavily, but these did not solve the specific problem I had which was to generate a Zod schema from a Rust type.

So for example, if I have this type:

```rust
struct Foo {
    name: String,
    id: u32,
    value: f64
}
```

This crate provides a way to automatically generate a Zod schema like so:

```ts
export const FooSchema = z.object({
    name: z.string(),
    id: z.number(),
    value: z.number(),
});

export type Foo = z.infer<typeof FooSchema>;
```

Toward that goal, this crate implements a procedural macro, which grants runtime type reflection to Rust types.

---

# Goals

This crate is opinionated.  It's intended to make it as easy as possible to make a pragmatic subset of all types expressable in Rust easily sharable with other languages, and TypeScript in particular.

The goals of this crate are:
- To provide a procedural derive macro which can make a Rust type portable to TypeScript with one line of code
- To interoperate with Serde for easy sharing of types between languages via JSON
- To give the developer control over when and where the types definitions are exported in different languages

Non-goals:
- This crate does not seek to support every rust type.  The goal is to support types which can easily be shared between languages.  If a type is not supported, the macro should fail fast with a meaningful error message.  Examples of unsupported types include:
    - Reference types (&)
    - Box, Ref, Arc, Mutex etc.
    - Basically anything which can't be easily serialzied into JSON
- This crate is not currently optimized for performance, it's optimized for productivity

---

# Important Details:

## Serde Attributes

The `Reflect` macro has support for certain `serde` attributes to make it easier to keep all representations aligned with the serialized representation.

Specifically this includes:

### 1. rename_all

This attribute is commonly used to convert between case conventions, like `snake_case` and `camelCase` for keys.

So for example, a rust `snake_case` representation can be converted to `camelCase` in the Zod output by using this attribute:

```rust
#[derive(Reflect, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Foo {
    key_1: ...
    key_2: ...
}
```

would result in the `Zod` export using the key names `key1` and `key2`.

### tag

For enum types with associated data, the `tag` attribute is requires.  So for instance this declaration:

```rust
#[derive(Reflect, Serialize, Deserialize)]
enum MyEnum {
    VariantA { x: u32 }
    VariantB { text: String }
}
```

will throw an error.  The reason for this is that by default, serde uses externally tagged JSON representation of enums.  I.e. the above code would serialize to:

```json
{ "VariantA": { "x": 42 }}
{ "VariantB": { "text": "foo" }}
```

This type of enum representation is disallowed by `type_reflect` because it is less convenient to bridge to typescript union types, which are the best analog for ADT's in typescript.

## Emitters Must Implement `Default`

Due to the way emitters are instantiated by `export_types!`, it's requried for emitters to implement the `Default` trait if they are declared as a destination in the macro.

So for example this destination:

```
MyEmitter(...)
```

desugars to this:

```
let mut MyEmitter {
    ..Default::default()
}
```

This is required to support passing named parameters to an emitter with defaults.

`Default` was not made a requirement of the trait, in order to support creating an emitter instance outside the context of the macro.

---

# Example Usage:

A working example can be found at [`type_reflect/examples/declare_and_export.rs`](type_reflect/examples/declare_and_export.rs).

It can be run with the command: `cargo run -p type_reflect --example declare_and_export`

the output files will be written to: `type_reflect/example_output`

## Simple Struct Definition:

```rust
#[derive(Reflect)]
struct MyStruct {
    foo: i32,
    bar: String,
    baz: MyOtherType
}


export_types!(
    types: [
        MyStruct,
        MyOtherType,
    ]
    exports: [
        Zod("/export/dir/1"),
        Rust("/export/dir/2", "/export/dir/3"),
        MyCustomExport("/export/dir/4")
    ]
)
```

Where `export_types` desugars to:

```rust
let mut emitter = Zod {
    ..Default::default()
};
let mut file = emitter
    .init_destination_file(
        "/export/dir/1",
        "",
    )?;
file.write_all(emitter.emit::<MyStruct>().as_bytes())?;
file.write_all(emitter.emit::<MyOtherType>().as_bytes())?;
emitter.finalize("/export/dir/1")?;
...
```

Here all directories are relative to the current working director from which the binary is executed.

## Custom Prefix

It's also possible to support a custom prefix for output files.

This may be useful, for instance, if we want to have an exported type depend on a type defined directly in the destination project.

For instance, let's say we have a type defined and exported like so:

```
#[define(Reflect, Serialize, Deserialize)]
struct Foo {
    bar: Bar
}
...
export_types!(
    types: [
        Foo,
    ]
    exports: [
        TypeScript("./export/foo.ts"),
    ]
)
```

By default, this will result in the following `.export/foo.ts` being generated:

```ts
export type Foo {
  bar: Bar
}
```

Of course this is not valid typescript, because the type `bar` here is undefined.

So we could add a prefix to import `Bar` from a different location:

```rs
export_types!(
    types: [
        Foo,
    ]
    exports: [
        TypeScript(
            "./export/foo.ts",
            prefix: "import { Bar } from './bar.ts'",
        ),
    ]
)
```

This will desugar like so:

```rs
let mut emitter = TypeScript {
    ..Default::default()
};
let mut file = emitter
    .init_destination_file(
        "/export/dir/1",
        "import { Bar } from './bar.ts'",
    )?;
```

And will generate the following typescript:

```ts
import { Bar } from './bar.ts'

export type Foo = {
  bar: Bar;
};
```

By default, the prefix will be added to the output file *before* the `emitter.dependencies()`, but this can be customized within the `TypeEmitter` implementation.

## Custom TypeEmitter Arguments

Through the `export_types` macro, it's also possible to forward initialization arguments to a type emitter.

So for instance, the `TypeScript` emitter supports a `tab_size` argument to define the output tab size.

So if the argument were specified like so in the `export_types` function:

```rust
    ...
    exports: [
        TypeScript(
            "/export/dir/1",
            tab_size: 4,
        ),
    ]
    ...
```

This would be desugared like so:

```rust
let mut emitter = TypeScript {
    tab_size: 4,
    ..Default::default()
};
```

The `prefix` argument is not forwarded for emitter initialization, since it's passed to the call for `init_destination_file`.

## Multi-emitter Destinations

It's also possible to define destinations in `export_types` with multiple emitters.  This might be useful, for instance, if you want to use mutluple type emitters to output to the same file.  For instance:

```rs
export_types! {
    types: [
        Foo,
        Bar,
        Baz,
    ],
    destinations: [
        (
            "./ouptu_file.ts",
            emitters: [
                TypeScript(),
                TSValidation(),
            ]
        ),
    ]
}
```

This would first emit the types `Foo`, `Bar` and `Baz` using the `TypeScript` emitter, and then using the `TSValidation` emitter.

## Enum Transformations

How an enum is transformed depends on the type of enum.

*Simple enums*, which are defined as enums without associated data, will simply be transformed into typescript enums with string values.

So for examle this enum:

```rust

enum Foo {
    Bar,
    Baz
}

```

will emit the following:

```ts

export enum SimpleEnumsExample {
    Foo = "Foo",
    Bar = "Bar,
}

export const SimpleEnumsExampleSchema = z.enum([
    SimpleEnumsExample.Foo,
    SimpleEnumsExample.Bar,
])

```




Enums with associated data are transformed by default to unions, with the `_case` field used to differntiate the cases.

So for instance this enum:

```rust
#[derive(Debug, Reflect, Serialize, Deserialize)]
#[serde(tag = "_case", content = "data")]
enum Status {
    Initial,
    #[serde(rename_all = "camelCase")]
    InProgress {
        progress: f32,
        should_convert: bool,
    },
    Complete {
        urls: Vec<String>,
    },
}
```

will be emitted as:

```ts

export enum StatusCase {
    Initial = "Initial",
    InProgress = "InProgress",
    Complete = "Complete",
}

export const StatusCaseInitialSchema = z.object({
    _case: z.literal(StatusCase.Initial),
});
export type StatusCaseInitial = z.infer<typeof StatusCaseInitialSchema>

export const StatusCaseInProgressSchema = z.object({
    _case: z.literal(StatusCase.InProgress),
    data: z.object({
        progress: z.number(),
    shouldConvert: z.bool(),
    })});
export type StatusCaseInProgress = z.infer<typeof StatusCaseInProgressSchema>

export const StatusCaseCompleteSchema = z.object({
    _case: z.literal(StatusCase.Complete),
    data: z.object({
        urls: z.array(z.string()),
    })});
export type StatusCaseComplete = z.infer<typeof StatusCaseCompleteSchema>

export const StatusSchema = z.union([
    StatusCaseInitialSchema,
    StatusCaseInProgressSchema,
    StatusCaseCompleteSchema,
]);
export type Status = z.infer<typeof StatusSchema>

```
