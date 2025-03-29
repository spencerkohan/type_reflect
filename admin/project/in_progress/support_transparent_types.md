# Support Transparent Types

Certain types are treated as transparent when serializing and deserialzing a type using Serde.

So for instance, if we have a type like so:

```
struct Foo {
    val: Box<bool>
}

let foo = Foo { Box::new(true) }
```

This will be serialized like so:

```
{
    "val" : true
}
```

We can add support for the following transparent types:

- Box
- Rc
- Arc
- Mutex
- RwLock


Todo:

- [ ] Add type representation for transparent types
- [ ] Parse transparent types from proc macro input
- [ ] Handle transparent types in the generators
    - [ ] typescript
    - [ ] ts_validation

- For now we omit zod, because it's a bit harder to implement consistently
