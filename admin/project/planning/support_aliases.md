# Type Alias Support

*As a developer, I want to be able to delcare a type alias in Rust and use it in TS*

## Design

A type alias in Rust:

    type Foo = String;

Should output like so:

    export const FooScema = z.string();
    export type Foo = z.infer<typeof FooScema>

## Challenge

In order to support this, I need to be able to run Reflect outside the context of a derive macro.  Derive macros apparently don't work on type declarations in Rust.
