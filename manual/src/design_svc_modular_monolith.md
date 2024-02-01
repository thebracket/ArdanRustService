# Combining Services into a Modular Monolith

Take a quick look at the `deploy_bookstore` project. There's a second service: `bookstore`. It follows the exact same template, and is entirely independent---other than using the `auth_layers` layer. (As I said before, that'll have to be separated if you need to de-monolith later. Be wary of adding coupling like this!)

Our `main.rs` file is deliberately simple and easy to maintain. We start by including each service as a module:

```rust
mod auth;
mod bookstore;
```

Then we include our own `mod service_config`, which follows the same template as services but defines the top-level service attributes:

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceConfig {
    pub listen_address: String,
    pub listen_port: String,
    pub static_content: String,
}
```

`static_content` defines the directory to serve as static files. You don't always have those.

Finally, the `main` function initializes each service and adds them into a single webserver:

```rust
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let service_settings = service_config::ServiceConfig::load()?;
    let auth_router = auth::setup_service().await?;
    let books_router = bookstore::setup_service().await?;

    // Listen address from configuration
    let listen_address = format!(
        "{}:{}",
        service_settings.listen_address, service_settings.listen_port
    );
    let listener = tokio::net::TcpListener::bind(&listen_address).await?;
    tracing::info!("Listening on {}", listen_address);

    // The default web server
    let static_content = ServiceBuilder::new()
        .layer(CorsLayer::very_permissive())
        .service(ServeDir::new(&service_settings.static_content));

    // Build the master router
    let master_router = axum::Router::new()
        .layer(CorsLayer::very_permissive())
        .nest("/api/v1/auth", auth_router)
        .nest("/api/v1/books", books_router)
        .layer(Extension(service_settings))
        .nest_service("/", static_content);

    // Launch Axum
    axum::serve(listener, master_router).await?;

    Ok(())
}
```

You may need to make this more complicated, but try not to: having a single, *simple* call site makes it easier to grow your application:

* Services are separated. You only need to call the configuration and nest/merge the `Router`.
    * If you're using `gRPC`, you can spawn that here, too.
* Even though you have a single program, it's easy to work on as a team:
    * You can easily define responsibility areas within the program.
    * You can easily divide services out into separate crates (and separate repos if you like), to keep teams from treading on one another.
    * You've maintained an API approach---you aren't relying on the "innards" of each module, just what's exposed.