# Per-Service Database

Not every service will *have* a database---you might use files, just perform computations. A *lot* of services will, so it's worth mentioning.

Some guidelines:
* Start by dividing your database into a separate module. 
* Expose an API for the rest of your program. 
* Don't litter your control code with direct database calls. Use the API, even internally.

Once you start to scale out, you will also want to:
* Separate the API for *calling* the database from any model (e.g. `FromRow` if you use `sqlx`) structures. You may want to reuse the model elsewhere without having to write a second layer of "data transfer objects" and spend time converting between the two.
* Consider making migrations a separate task. You want to be *really* careful about having multiple instances of a system changing your database!

Let's have a look at the database layer for the `auth` system. It's quite simple, but follows a common template:

## Strong/New Typed Database Pool

```rust
#[derive(Clone)]
pub struct AuthDb(pub sqlx::SqlitePool);
```

You don't want confusion with lots of extensions all trying to include a `SqlitePool` (or similar). You can avoid this by introducing a new type that unambiguously represents your service's database resource.

## Obtain Connection Pool

```rust
pub async fn get_connection_pool(filename: &str) -> Result<AuthDb> {
    let options = SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(true);

    let connection_pool = sqlx::SqlitePool::connect_with(options)
        .await?;
    Ok(AuthDb(connection_pool))
}
```

You'll typically need to call this once during service setup and share the pool as an extension layer. If you are using something other than Axum and need to have it *also* be available to other services create a static variable to share it within your module---but *do not* expose that static outside of your module.

## Perform Migrations

If you are using migrations, you can embed your services migrations in the binary:

```rust
pub async fn perform_migrations(db_pool: AuthDb) -> Result<()> {
    sqlx::migrate!("src/auth/migrations")
        .run(&db_pool.0)
        .await?;
    Ok(())
}
```

## API functions

The service also includes functions to:
* `login` - take a username and password, return a `Result<Option<UserId>>`.
* `add_token` - creates a random token for a user ID.
* `get_user_id_from_token` - checks that a token exists, and if it does returns the user ID.
* And the regular CRUD (Create/Read/Update/Delete): `get_all_users`, `get_user`, `delete_user`, `update_user`

There's only one "model" type: `User`.

The takeaway is that the database is an API, and self-contained. You can easily scale it up: first into a directory-based module, and then into its own crate---should you need to.