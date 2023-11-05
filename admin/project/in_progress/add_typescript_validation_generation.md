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

## TODO:

- [ ] Implement generation for struct types
- [ ] Implement generation for enum types
    - [ ] Enum variants
    - [ ] Enum union type
