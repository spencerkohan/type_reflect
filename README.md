# Type Reflect

*Be aware: the current status of this project is very incomplete.  I'm using it as

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



## Example Usage:

Type declaration:

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
    Double(i32, f32),
    Single(i32),
}
```

will be emitted as:

```

export enum StatusCase {
    Initial = "Initial",
    InProgress = "InProgress",
    Complete = "Complete",
    Double = "Double",
    Single = "Single",
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

export const StatusCaseDoubleSchema = z.object({
    _case: z.literal(StatusCase.Double),
    data: z.tuple([
            z.number(),
        z.number(),
    ])});
export type StatusCaseDouble = z.infer<typeof StatusCaseDoubleSchema>

export const StatusCaseSingleSchema = z.object({
    _case: z.literal(StatusCase.Single),
    data: z.number()});
export type StatusCaseSingle = z.infer<typeof StatusCaseSingleSchema>


export const StatusSchema = z.union([
    StatusCaseInitialSchema,
    StatusCaseInProgressSchema,
    StatusCaseCompleteSchema,
    StatusCaseDoubleSchema,
    StatusCaseSingleSchema,
]);
export type Status = z.infer<typeof StatusSchema>

```
