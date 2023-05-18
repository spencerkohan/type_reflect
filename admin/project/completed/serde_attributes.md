# Serde Attributes

*As a developer, I want type_reflect to automatically observe the serde attributes when emitting my types, so that my typescript types can be created from the JSON produced by serializing my Rust types thorugh Serde.*

# Acceptance Criteria

- [x] For enum types, the serde "tag" attribute should be used as the case identifier

I.e. if I have an enum decalred like so:

```
#[derive(Reflect, Serialize)]
#[serde(tag="my_tag")]
enum SerdeTagExample {
    Foo { ... }
}
```

Then Foo should generate the following zod output:

```
export const SerdeTagExampleCaseFooSchema = z.object({
    my_tag: SerdeTagExampleCase.Foo,
    ...
});
export type SerdeTagExampleCaseFoo = z.infer<typeof SerdeTagExampleCaseFooScema>
```


- [x] For enum types, the serde "content" attribute should be used to as the key for nested data

So for instance, if we have this enum:

```
#[derive(Reflect, Serialize)]
#[serde(tag="my_tag", content="data")]
enum SerdeContentExample {
    Foo(i32)
}
```


Then Foo should generate the following zod output:


```
export const SerdeContentExampleCaseFooSchema = z.object({
    my_tag: SerdeContentExampleCase.Foo,
    content: z.number()
});
export type SerdeContentExampleCaseFoo = z.infer<typeof SerdeContentExampleCaseFooScema>
```


- [x] If the "tag" or "content" attribute is missing, an error should be thrown
