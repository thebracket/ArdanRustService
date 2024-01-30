# More Extractors

There are a lot more extraction options. See [https://docs.rs/axum/latest/axum/extract/index.html#applying-multiple-extractors](https://docs.rs/axum/latest/axum/extract/index.html#applying-multiple-extractors) for an exhaustive list!

* You can include `body: String` to receive the entire posted body as a string.
* You can include `body: Bytes` to receive the entire posted body as an array of bytes.
* You can include `request: Request` to receive the entire HTTP request!

You can also use the `Json` extractor. We'll use this in practice later, so we're not going to dwell on it now - but any `Deserializable` type (implementing Serde's trait) can be posted as JSON and automatically converted into a strong type.

A few more things to note:

* You can make an extractor *optional*. `Option<my extractor>` lets you still match the route if an extraction wasn't provided.
* You can wrap a strong typed extractor (such as Json) in `Result<Json<MyType>>` and check to see if deserialization worked---and see an error message telling you what went wrong.

We'll deal with *extensions* next.