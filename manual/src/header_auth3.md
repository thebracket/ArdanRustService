# Middleware Auth with Injection

> The code for this is in `code/rest_service/axum_header_layer`.

You probably want your functions to be able to know about the authenticated user. If they aren't authenticated, you are handling that---but what about passing information about them to handlers? Middleware request functions can *inject* extension data into the pipeline:

```rust
async fn auth(
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {    
    if let Some(header) = headers.get("x-request-id") {
        // Validate the header
        let header = header.to_str().unwrap();
        if header == "1234" {
            req.extensions_mut().insert(AuthHeader { id: header.to_string() });
            return Ok(next.run(req).await);                
        }
    }

    Err((StatusCode::UNAUTHORIZED, "invalid header".to_string()))
}
```

Making `req` mutable allows you to call `req.extensions_mut().insert` and add data to the request. Your handler can now retrieve that data:

```rust
async fn header_handler(
    Extension(auth): Extension<AuthHeader>) -> Html<String> {
    Html(format!("x-request-id: {}", auth.id))
}
```

You don't *have* to request that extension---so a function that just needs to know if its authorized can work as-is. But if you need some additional information about the user, you can pass it along.

> You'd probably authenticate via a function and return a structure with more information!