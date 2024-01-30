# The Config Crate

A popular crate for managing configuration is the `config` crate. It supports all of the scenarios we listed in the introduction, and can provide for very flexible configuration. It is battle tested and widely used.

The `config` crate supports reading configuration from a number of sources, and uses `serde` to map them into different formats.

Let's start with a minimal test.

> The code for this is in `code/config/config_minimal`.

We'll first make a project and add some dependencies:

```bash
cargo new config_minimal
cd config_minimal
cargo add dotenvy
cargo add config
```

Now let's edit `main.rs` to read config from several sources:

```rust
use std::collections::HashMap;
use config::Config;

fn main() {
    // Ignore the result of loading .env --- it's ok if it doesn't exist
    let _ = dotenvy::dotenv();

    let settings_reader = Config::builder()
        .add_source(config::File::with_name("settings").required(false))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let settings = settings_reader
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();

    println!("{settings:?}");
}
```

Now let's try running it a few ways.

* `cargo run` on its own will print an empty set of settings: `{}`
* `TEST=test cargo run` will print `{}`. Environment variables are only loaded if they are prefixed with `APP_` - that's what our `with_prefix` does.
* `APP_TEST=test cargo run` will print `{"test": "test"}`

Finally, let's add a file: `settings.toml`. We'll fill it out:

```toml
test="test"
```

And `cargo run` shows that it loaded the setting from the file: `{"test": "test"}`

You can mix and match. With the file still in place, try `APP_TEST2=test2 cargo run`---you will see `{"test": "test", "test2": "test2"}`.

You can also override file settings with environment variables. If you `APP_TEST=foo cargo run` you will see `foo` not `test` as the output. Environment variables have precedence by default.