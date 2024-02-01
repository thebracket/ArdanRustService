# Per-Service Configuration

The `auth` service has its own configuration. This is very common: most services will have configurable items, and you want to offer flexibility when running the program. The `auth` service configuration is very simple:

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthConfiguration {
    pub db_filename: String,
}
```

> Note that since we're using SQLite files, we just have a filename here. In reality, you've probably got one or more connection strings.

We also wrap loading the configuration into an associated method---a constructor:

```rust
impl AuthConfiguration {
    pub fn load() -> Result<Self> {
        // Load any .env files
        // Ignore the result of loading .env --- it's ok if it doesn't exist
        let _ = dotenvy::dotenv();

        let settings_reader = Config::builder()
            .add_source(config::File::with_name("settings").required(false))
            .add_source(config::Environment::with_prefix("AUTH"))
            .build()?;

        let settings = settings_reader
            .try_deserialize()?;

        Ok(settings)
    }
}
```

This is more-or-less boilerplate---it'll be the same for most services. We:
1. Load any `.env` files.
2. Use the `Config` crate to optionally load a settings file, and apply any settings from environment variables prefixed with `AUTH_`. In this case, `AUTH_DB_FILENAME`.
3. We deserialize into a strongly typed settings type. If essential configuration items are missing or malformed, the whole process will bail out with an error telling you "Missing db_filename" or similar. Give your environment clues as to what broke!
