# gRPC Streaming - The Client

This follows a similar pattern:

```rust
use streaming::{streaming_client::StreamingClient, Start};

pub mod streaming {
    tonic::include_proto!("streaming");
}

#[tokio::main]
async fn main() {
    let mut client = StreamingClient::connect("http://[::1]:10000").await.unwrap();
    for n in 1..10 {
        println!("Requesting squares up to {}", n);
        let request = tonic::Request::new(Start { n });
        let mut stream = client.squares(request).await.unwrap().into_inner();
        while let Some(result) = stream.message().await.unwrap() {
            println!("RESULT={:?}", result);
        }
    }
}
```

We include our protocol, and then call it---and treat the results exactly like a normal channel.

You can test this by:
* In one window, run `cargo run --bin streaming-server`
* In a second window, run `cargo run --bin streaming-client`

Even in debug mode, it's very fast! You are maintaining one HTTP connection, rather than spawning a new one for each command.