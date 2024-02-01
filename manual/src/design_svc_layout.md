# Service Layout

Picking a good structure up-front can save you from hours of painful refactoring (although Rust makes refactoring easy!). Many of the same guidelines apply at all scales.

## Module vs Crate

An individual service should be either a module (`mod my_service` pointing at a directory) or a crate (`cargo new my_service` and include the service in your dependencies). As it grows, you may want to break the service into *multiple* crates both for compile time and for ease of integration.

We'll talk about modules---but remember, you can scale them out as needed into crates.

## Basic Layout

In the `no_workspace/deploy_bookstore` example, we've got two services: `auth` and `bookstore`. Each is self-contained inside a module---and could easily become one or more separate crates if that module needs to be isolated (for political or scaling purposes). There's also some static HTML.

So the basic layout of the program is:

```
-- src
    -- auth
    -- bookstore
    -- static_html
```

The goal here is to allow each of the modules to be self-contained with very little coupling to other services.

And then inside each of the service modules, you'll find a layout like this:

```
-- auth
    -- migrations (containing SQLX database migrations)
    -- db (containing the data layer)
    -- configuration (per-service configuration)
    -- web_service (Axum specific code)
    mod.rs -- exposes a "setup service" function that returns a Router
```

The `auth` module also includes an `auth_layers` module---which is shared with other components. We'll talk about that in a bit.

Here's the full `setup_service` function for `auth`:

```rust
pub async fn setup_service() -> Result<Router> {
    let config = configuration::AuthConfiguration::load()?;
    let db_pool = db::get_connection_pool(&config.db_filename).await?;

    db::perform_migrations(db_pool.clone()).await?;

    let secure_router = Router::new()
        .route("/users", get(web_service::list_users))
        .route("/users/:id", get(web_service::get_user))
        .route("/users/delete/:id", get(web_service::delete_user))
        .route("/users/add", post(web_service::add_user))
        .route("/users/update/:id", post(web_service::update_user))
        .layer(Extension(config.clone()))
        .layer(Extension(db_pool.clone()))
        .route_layer(middleware::from_fn(auth_layers::require_token));

    let router = Router::new()
        .route("/login", post(web_service::do_login))
        .nest("/", secure_router)
        .layer(Extension(config))
        .layer(Extension(db_pool));

    Ok(router)
}
```

Let's walk through this:
1. We use a `Result` type to ensure we can communicate errors to the parent system.
2. We load per-service configuration.
3. We initialize our database connection pool.
4. We perform any migrations (be careful with this in production!)
5. We build a `secure_router` layer that attaches the configuration, database pool, and a `require_token` middleware layer.
6. We build a router with publicly available routes, and also inject configuration and the database pool.
7. We return the router.

What *isn't* here? We don't initialize Axum, start listening to the network. That will be handled at the top-level---allowing us to easily host a multi-tenant service architecture.

We've designed the beginnings of a modular monolith.