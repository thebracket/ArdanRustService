# Loading Config via HTTP

HTTP configuration is a bit more complicated, because it will use an HTTP(s) source asynchronously. `Reqwest` isn't baked directly into `config`, so we have to provide a helper. This is very flexible because you can customize the request to match your requirements. It's also a bit more work.

> The code is in `code/config/config_http`.

Let's create another project:

```bash
cargo new config_http
cd config_http
cargo add dotenvy
cargo add config
cargo add tokio -F full
cargo add axum
cargo add reqwest -F json
cargo add async_trait
```

This example is a little larger:

```rust
use async_trait::async_trait;
use axum::Router;
use axum::routing::get;
use config::{AsyncSource, Config, ConfigError, Format, Map, FileFormat};
use std::collections::HashMap;
use std::fmt::Debug;
use std::time::Duration;

#[tokio::main]
async fn main() {
    tokio::spawn(load_settings());
    test_server().await;    
}

async fn test_server() {
    // Fire up a minimal Axum server to provide some settings
    let app = Router::new().route("/", get(|| async {
        "test_setting = \"test\""
    }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn load_settings() {
    tokio::time::sleep(Duration::from_secs(1)).await;
    // Ignore the result of loading .env --- it's ok if it doesn't exist
    let _ = dotenvy::dotenv();

    let settings_reader = Config::builder()
        .add_source(config::File::with_name("settings").required(false))
        .add_source(config::Environment::with_prefix("APP"))
        .add_async_source(HttpSource {
            uri: "http://localhost:3001/".into(),
            format: FileFormat::Toml,
        })
        .build()
        .await
        .unwrap();

    let settings = settings_reader
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();

    println!("{settings:?}");
}

#[derive(Debug)]
struct HttpSource<F: Format> {
    uri: String,
    format: F,
}

#[async_trait]
impl<F: Format + Send + Sync + Debug> AsyncSource for HttpSource<F> {
    async fn collect(&self) -> Result<Map<String, config::Value>, ConfigError> {
        reqwest::get(&self.uri)
            .await
            .map_err(|e| ConfigError::Foreign(Box::new(e)))? // error conversion is possible from custom AsyncSource impls
            .text()
            .await
            .map_err(|e| ConfigError::Foreign(Box::new(e)))
            .and_then(|text| {
                self.format
                    .parse(Some(&self.uri), &text)
                    .map_err(ConfigError::Foreign)
            })
    }
}
```

There's three parts to this program:

* The `test_server` function runs an HTTP server, so the demo has a source of configuration data. You normally don't need this.
* The `load_settings` function is what you need as a client. It's relatively straightforward---you've added an async source.
* The `HttpSource` system would have to be customized to work with your configuration file source, which is why it isn't baked into `Config`. You can write this once, share it as a library for your services.

