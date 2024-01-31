# A native WS client

Axum is using a library called "Tungstenite" behind the scenes for websocket handling. Tungstenite also includes a client. We'll use the same server as we did for the last example, so start that running.

> The code for this is in `code/wss/ws_client`.

Let's start with a new project and some dependencies:

```bash
cargo new ws_client
cd ws_client
cargo add tokio -F full
cargo add futures_util # We're using this for stream helpers
cargo add tokio-tungstenite
```

Then the code is very much like working with a regular stream:

```rust
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut ws_stream, _) = tokio_tungstenite::connect_async("ws://localhost:3001/ws").await?;
    let message = Message::Text("Hello".to_string());
    ws_stream.send(message).await?;

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        println!("Received: {}", msg);
    }

    Ok(())
}
```

Run this to connect to our echo server, and it will send "Hello" --- and receive the same. You'll have to `ctrl-C` to exit, we didn't include a bail-out system!