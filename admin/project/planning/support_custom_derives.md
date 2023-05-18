# Custom Derives

*As a developer, I might want my exported types to contain custom derive macros in Rust*

## Design

I would like to be able to define a rust output with custom derives like so:

```

exort_types! {
    ...
    destinations: [
        Rust(
            derrives: [
                MyCustomMacro
            ],
            ...
        )
    ]

}


```
