# gRPC Streaming - The Server

Rename `main.rs` to `server.rs`. We'll start by including some types we will use:

```rust
use tonic::{transport::Server, Request, Response, Status};
use tokio_stream::wrappers::ReceiverStream;
```

Next, we include our generated code and the types it created for the server:

```rust
pub mod streaming {
    tonic::include_proto!("streaming");
}
use streaming::streaming_server::{ Streaming, StreamingServer};
use streaming::{Start, Square};
```

Now we define our streaming service type:

```rust
#[derive(Debug, Default)]
pub struct StreamingService {}
```

Next, we implement the `Streaming` trait Tonic generated to match our protocol.

```rust
#[tonic::async_trait]
impl Streaming for StreamingService {
    type SquaresStream = ReceiverStream<Result<Square, Status>>;

    async fn squares(&self, request: Request<Start>) -> Result<Response<Self::SquaresStream>, Status> {
        println!("Got a request: {:?}", request);
        let (tx, rx) = tokio::sync::mpsc::channel(4);

        tokio::spawn(async move {
            for i in 0..request.into_inner().n {
                let square = Square {
                    n: i * i,
                };
                tx.send(Ok(square)).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
```

Notice that we've defined a local type: `SquaresStream`. The `ReceiverStream` syntax is really unwieldy, so let's not type it over and over. The `squares` function receives a request, and returns a `SquaresStream`---wrapped in a `Result` in case anything goes wrong.

We then create a channel, just like in *Rust Foundations*. We spawn an async task---this will run independently. We then send the receiver part of the channel back to the client. 

Inside the spawned task, we use the regular channel syntax to submit results.

Finally, we need to actually start the server in our `main` function:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();
    println!("Square Server listening on: {}", addr);

    let streamer = StreamingService {};
    let svc = StreamingServer::new(streamer);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
```