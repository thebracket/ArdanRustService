# Layers and API Systems for Other Systems

The `auth` system also includes some middleware that is used by both the `auth` service itself and by other parts of the program. This is where things get potentially sticky.

The `auth_layers` module is shared with `pub mod`---it's available to the rest of the program. This presents a future scaling hazard: if you move parts of the system outside of a single monolithic service executor, but still want to use a layered approach that remains the responsibility of the `auth` system you'd want to:

1. Create a separate `auth_client_layers` crate.
2. Implement an RPC (REST, gRPC, etc.) system in that crate that *calls* the `auth` API over the network.
3. Uses that code instead inside the layer to provide the same functionality.

The layer injects a `ValidUser` type into recipients that request it:

```rust
#[derive(Clone, Copy, Debug)]
pub struct ValidUser(pub i32);
```

The middleware itself is called `require_token`, and follows the system we discussed in Axum layers:

```rust
pub async fn require_token(
    Extension(db_pool): Extension<db::AuthDb>,
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Some(auth_header) = headers.get("Token") {
        let token = auth_header.to_str().map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "invalid header".to_string(),
            )
        })?;

        if let Some(user_id) = db::get_user_id_from_token(db_pool, token)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database error".to_string(),
                )
            })?
        {
            req.extensions_mut().insert(ValidUser(user_id));
            return Ok(next.run(req).await);
        }
    }

    Err((StatusCode::UNAUTHORIZED, "invalid header".to_string()))
}
```

1. We intercept a Request.
2. We read the headers.
3. If we find the "Token" header:
    * We check that it is valid and return the user id.
4. If we didn't find the token, or it is invalid we return an error.