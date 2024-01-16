# Using IntoResponse

Now you've seen a bit of Axum's flexibility. You can inject dependencies, and return a variety of Response types---`Html()`, `Json()`, `StatusCode` etc. 
Understanding a little about how this works under the hood will help you write much more ergonomic code.

All of the response types you've used implement an `IntoResponse` trait. That is, the `Response` type can be constructed from a wide variety
of types.

You can also take advantage of this. Your handlers can just return `impl IntoResponse` and infer the actual return type from the function
body. This makes for quicker writing, but it isn't as obvious---so we've not used it thus far.

For example, you can declare handlers like this:

```rust
fn handler() -> impl IntoResponse {
    Html("<h1>Big text!</h1>")
}

fn handler2() -> impl IntoResponse {
    Json(42)
}
```

That's not a huge win, but it does open up some possibilities.
