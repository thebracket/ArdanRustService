# Extractors

In the minimal example, your route is controlled with this line of code:

```rust
.route("/", get(handler));
```

The matching handler runs with no parameters:

```rust
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
```

The `handler()` portion has no parameters - it's not requesting any additional information. The `Html<>` represents a *Response*---we'll talk about customizing responses later.

But what if you *want* to know more about the request? Axum provides *extractors*---which run as part of the `Request` portion of the lifecycle.