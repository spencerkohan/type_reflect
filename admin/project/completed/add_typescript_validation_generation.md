# Typescript Validation & Parsing Generation

I've tried integrating with [`typia`](https://typia.io/docs/) for a smoother experience with runtime type validation, but I am running into the blocker that the code transformation required by Typia does not work well with the compilation process implemented by Vite.

Therefore, since I am implementing code generation anyway, I can create my own ahead-of-time code generation for the types I generate.

## Design

For each type I generate, I want to generate two functions:
1. A parsing implementation
2. A validation implementation

So for example if I have this type:

```rs
struct Foo {
    x: f32,
    name: String,
    bar: Bar,
}
```

This should generate the following functions:

```ts
namespace Foo {
  export function validate(input: any): ValidationResult {
    if(!input.x) {
      return {ok: false, error: "Error vaildaing Foo: expected member x does not exist"};
    } else {
      let res = validateNumber(input.x);
      if(!res.ok) {
        return {ok: false, error: `Error vaildaing Foo: ${res.error}`
      }
    }

    if(!input.name) {
      return {ok: false, error: "Error vaildaing Foo: expected member name does not exist"};
    } else {
      let res = validateString(input.name);
      if(!res.ok) {
        return {ok: false, error: `Error vaildaing Foo: ${res.error}`
      }
    }

    if(!input.bar) {
      return {ok: false, error: "Error vaildaing Foo: expected member bar does not exist"};
    } else {
      let res = Bar.validate(input.bar);
      if(!res.ok) {
        return {ok: false, error: `Error vaildaing Foo: ${res.error}`}
      }
    }
    return {ok: true};
  }

  export function parse(input: string): ParseResult<Foo> {
    const data = JSON.parse(input);
    let val = Foo.validate(data);
    if(!val.ok) {
      return val;
    }
    return {
      ok: true,
      value: data as Foo,
    }
  }
}

```

### Array Validation

For an array, we need to validate that every member of the array conforms to the desired type

```ts

type ArrType = {
    records: Array<Foo>
}

namespace ArrType {
  function validate(input: any) -> Result<ArrType> {
    if(!input.records) {
      return {ok: false, error: `Error vaildaing ArrType: expected ArrType.records to be defined`}
    } else {
      if(!Array.isArray(input.records)) {
        return {ok: false, error: `Error vaildaing ArrType: expected ArrType.records to be an Array`}
      }
      for (let value in input.records) {
        let res = Foo.validate(input.bar);
        if(!res.ok) {
          return {ok: false, error: `Error vaildaing ArrType: ${res.error}`}
        }
      }
    }
  }
}

```

### Redesign

After giving this some thought, I think it's better that the validator should throw errors rather than using monads.

This is more idiomatic Typescript, and may even be more performant.

So each type generated should generate:

```ts
namespace MyType {
  // A validator which throws
  export function tryValidate(): MyType { ... }

  // A parser which throws
  export function tryParse(input: string): MyType { ... }

  // A validator which returns a result
  export function validate(): Result<MyType> { ... }

  // A parser which returns a result
  export function parse(input: string): Result<MyType> { ... }
}
```

### Error Types

For the thrown errors, we have to cover the follwing:

1. Missing members:
    - "Error validating MyType.member: expected [string] found [undefined]"

2. Type mismatch:
    - "Error validating MyType.member: expected [string] found [number]


### Array of Types

Each validator should also have the option to parse or validate an array of that type, for convenience

## TODO:

- [x] Implement generation for struct types
    - [x] Named type keys
    - [x] string keys
    - [x] number keys
    - [x] bool keys
    - [x] option keys
    - [x] Array keys
    - [x] Map keys
- [x] Implement generation for enum types
    - [x] Simple enums
    - [x] Enum variants
    - [x] Enum union type

- [x] Add tests
    - [x] Simple value
        - [x] validation
        - [x] parsing
    - [x] Optional member
    - [x] Nested types
    - [x] Arrays
        - [x] Type with array
        - [x] Array of types
    - [x] Map member
