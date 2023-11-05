# Type Emitter State

It would be nice to support state in type emitters, to allow for more flexibility in terms of how clients are able to generate types.

## Design

Currently, `export_types!`:

```rust
export_types!(
    types: [
        ServerEvent,
        ClientEvent,
        IdentifierIDRecord,
    ],
    destinations: [
        TypeScript(
            "./inspector_client/src/external_types/lsp_broker.ts",
            prefix: "export type IdentifierID = number;"
        ),
    ]
)
}
```

unwraps to something like this:

```rust
let mut file = TypeScript::init_destination_file(
    "./inspector_client/src/external_types/lsp_broker.ts",
    "export type IdentifierID = number;",
)?;
file.write_all(TypeScript::emit::<ServerEvent>().as_bytes())?;
file.write_all(TypeScript::emit::<ClientEvent>().as_bytes())?;
file.write_all(TypeScript::emit::<IdentifierIDRecord>().as_bytes())?;
TypeScript::finalize("./inspector_client/src/external_types/lsp_broker.ts")?;
```

So in other words, we have only associated functions to handle the type emission process.

It would be more powerful to pass an object to the export function, so that emitter could retain its own stat:

```rust
let mut file = TypeScript::init_destination_file(
    "./inspector_client/src/external_types/lsp_broker.ts",
    "export type IdentifierID = number;",
)?;

let mut emitter = TypeScript {..Default::default()};

emitter::emit<ServerEvent>(file)?;
emitter::emit<ClientEvent>(file)?;
emitter::emit<IdentifierIDRecord>(file)?;
emitter::finalize("./inspector_client/src/external_types/lsp_broker.ts")?;
```

Here we can also support the addition of forwarding of parameters to the emitter.

So for instance, we could have a destination which looks like this:

```rust
TypeScript(
    "./inspector_client/src/external_types/lsp_broker.ts",
    prefix: "export type IdentifierID = number;"
    indent_size: 2,
),
```

Generate this:

```rust
let mut emitter = TypeScript {
    indent_size: 2,
    ..Default::default()
};
```

# TODO:

- [x] Make emitters stateful
- [x] Forward named arguments to emitters
