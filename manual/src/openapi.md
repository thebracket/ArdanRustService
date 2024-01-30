# OpenAPI Documentation

You probably want to document your API, especially if you are sharing it with others. OpenAPI has become a de-facto standard for API documentation. Generating all of the description by hand would be tedious, so why not bake it into your service?

You'll need to add a few dependencies:

```bash
cargo add tokio -F full # Tokio
cargo add axum # The webserver
cargo add serde -F derive # Needed for the JSON type
cargo add utoipa
cargo add utoipa-redoc -F axum # For redoc format
cargo add utoipa-swagger-ui -F axum # It supports multiple webservers
```

The code for this is in `code/openapi/axum_openapi`.

```rust
use axum::{Json, routing::get, Router};
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use utoipa_redoc::{Redoc, Servable};

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            handler,
        ),
        components(
            schemas(HelloWorld)
        ),
        modifiers(),
        tags(
            (name = "Test System", description = "A really simple API")
        )
    )]
    struct ApiDoc;

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, ToSchema)]
struct HelloWorld {
    message: String,
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Say Hello to the World", body = [HelloWorld])
    )
)]
async fn handler() -> Json<HelloWorld> {
    Json(
        HelloWorld { message: "Hello, World!".to_string() }
    )
}
```

Now you can run the program as normal. Open up `http://localhost:3001/swagger-ui/` and you have a fully formed Swagger UI defining your project.

> You absolutely should still use Rust Documentation as well!