# Type Alias Support

*As a developer, I want to be able to delcare a type alias in Rust and use it in TS*

## Design

A type alias in Rust:

    type Foo = String;

Should output like so:

    export const FooScema = z.string();
    export type Foo = z.infer<typeof FooScema>
