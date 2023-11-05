# Mult-emitter destinations

It might be the case that I want to use multiple emitters for the same output file.

For instance, this can be helpful for generating TS validation:
- First emit the Typescript types, using the TS emitter
- Then emit the validaton functions, using the TSValidation emitter

## Design

I can support this in the `export_types` macro, with the following pattern:

```
export_types! {
    types: [ Foo ],
    destinations: [
        (
            "path/1.ts",
            "path/2.ts",
            prefix: "",
            emitters: [
                TypeScript(),
                TSValidation(),
                TSFormat(tab_size: 2)
            ]
        )
    ]
}
```

I.e. we introduce an "un-named" variant to the destination type, and allow multiple emitters to be passed.

In this case, the code generated should be:

```

let mut file =


let mut emitter = TypeScript {
    ..Default::default()
};
let mut file = emitter
    .init_destination_file(
        "/export/dir/1",
        "",
    )?;
file.write_all(emitter.emit::<Foo>().as_bytes())?;
emitter.finalize("/export/dir/1")?;

let mut emitter = TSFormat {
    tab_size: 2,
    ..Default::default()
};
file.write_all(emitter.emit::<Foo>().as_bytes())?;
emitter.finalize("/export/dir/1")?;
let mut emitter = TSValidation {
    ..Default::default()
};
file.write_all(emitter.emit::<Foo>().as_bytes())?;
emitter.finalize("/export/dir/1")?;
```


## TODO:
- [x] Move file and prefix creation out of emitter
- [x] Define Un-named emitter variant
    - [x] Define parsing behavior
    - [x] Define code generation
