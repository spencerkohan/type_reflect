# Type Reflect

This is a library intended to be used to provide runtime reflection for Rust types.

It also contains the functionality to generate typescript types, as well as zod scemas from rust types.

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
        Custom<MyExport>()
    ]
)
```

Where `export_types` desugars to:

```
let export_dir1 = "/export/dir/1"
remove_file(export_dir1)

export_dir1.append(MyStruct::export<Zod>())
export_dir1.append(MyOtherType::export<Zod>())

...
```

ToDo:

- [x] Basic end-to-end
    - [x] Parsing types implemented
    - [x] Export function working
- [ ] Support for complex types in TS/zod export
    - [ ] Option
    - [ ] Array
    - [ ] Map
- [ ] Support for Rust export
- [ ] Support for Enums
