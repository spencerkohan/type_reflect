# Type Reflect

***This project is currently fairly rough around the edges.  It's already usable in it's current state, but for instance the rust docs are WIP.  You may also expect to run into bugs and limitations, since so far it has only been tested on my own use-cases.  If you run into something, please report it so I can make this a better tool***

This library is implemented to make it easier to bridge between Rust types and Zod schemas for TypeScript.

This can be useful, for instance, in the context of a Rust Back-end service interacting with a TypeScript front-end.

Other solutions exist to solve similar problems, for instance the excellent [ts-rs crate](https://crates.io/crates/ts-rs), which I borrow from heavily, but these did not solve the specific problem I had which was to generate a Zod schema from a Rust type.

So for example, if I have this type:

```
struct Foo {
    name: String,
    id: u32,
    value: f64
}
```

This crate provides a way to automatically generate a Zod schema like so:

```
export const FooSchema = z.object({
    name: z.string(),
    id: z.number(),
    value: z.number(),
});

export type Foo = z.infer<typeof FooSchema>;
```

Toward that goal, this crate implements a procedural macro, which grants runtime type reflection to Rust types.

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

# Important Details:

## Serde Attributes

The `Reflect` macro has support for certain `serde` attributes to make it easier to keep all representations aligned with the serialized representation.

Specifically this includes:

### 1. rename_all

This attribute is commonly used to convert between case conventions, like `snake_case` and `camelCase` for keys.

So for example, a rust `snake_case` representation can be converted to `camelCase` in the Zod output by using this attribute:

```
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

```
#[derive(Reflect, Serialize, Deserialize)]
enum MyEnum {
    VariantA { x: u32 }
    VariantB { text: String }
}
```

will throw an error.  The reason for this is that by default, serde uses externally tagged JSON representation of enums.  I.e. the abovev code would serialize to:

```
{ "VariantA": { "x": 42 }}
{ "VariantB": { "text": "foo" }}
```

This type of enum representation is disallowed by `type_reflect` because it is less convenient to bridge to typescript union types, which are the best analog for ADT's in typescript.

# Example Usage:

## Simple Struct Definition:

```
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
        Rust("/export/dir/1", "/export/dir/2"),
        Zod("/export/dir/3"),
        MyCustomExport("/export/dir/4")
    ]
)
```

Where `export_types` desugars to:

```
let export_dir1 = "/export/dir/1"
remove_file(export_dir1)

export_dir1.write_all(MyStruct::export<Zod>())
export_dir1.write_all(MyOtherType::export<Zod>())

...
```

Here all directories are relative to the current working director from which the binary is executed.

## Enum Transformations

How an enum is transformed depends on the type of enum.

*Simple enums*, which are defined as enums without associated data, will simply be transformed into typescript enums with string values.

So for examle this enum:

```
enum Foo {
    Bar,
    Baz
}
```

will emit the following:

```
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

```
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

```

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
