# Suppoort Tuple Structs

I'm running into a limitaiton with the current implementation, that Reflect doesn't support tuple-type structs.

I.e. I can support structs like this:

```
struct Foo {
    x: u32
}
```

but not like this:

```
struct Foo(u32);
```

## Design

I already support tuple type enum variants, so it should not be a huge leap to support tuple type structs

How can we adapt the design?

Currently we support high-level traits for Rust types:

```
trait StructType
trait EnumType
```

for Structs, currently we assume that we have a set of named members.

We need to expand this definition to suppprt anonymous structs, and also unit structs.

So for instance, we could differentiate structs by having a different trait for each struct type:

```
trait StructTypeNamed
trait StructTypeTuple
trait StructTypeUnit
```

Or we could differentiate the types within the member type returned by the struct:

```
trait StructType {
    fn members() -> StructMembersNamed | StructMembersTuple | StructMembersUnit
}
```

I think the right approach here is to re-use the `EnumCaseType` here to represent all cases of type fields.

We can rename this to `TypeFieldDefinition`

## TODO:

- [x] Rename `EnumCaseType` to `TypeFieldDefinition`
- [x] Rename `TypeFieldDefinition::Simple` to `TypeFieldDefinition::Unit`
- [x] Rename `TypeFieldDefinition::Struct` to `TypeFieldDefinition::Named`
- [x] Rename `StructMember` to `NamedField`
- [x] Rename `StructType.members` to `StructType.fields`
