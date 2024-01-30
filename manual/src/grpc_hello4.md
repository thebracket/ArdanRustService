# Hello Tonic - The Client

The client starts in the same way: we import our generated code, and use some of it:

```rust
pub mod hello_world {
    tonic::include_proto!("hello");
}

use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
```

Then we make a Tokio main function, and call the client:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
```

Let's test it!

1. In one window, run `cargo run --bin helloworld-server`.
2. In a second window, run `cargo run --bin helloworld-client`.

The server will show:

```
Got a request: Request { metadata: MetadataMap { headers: {"te": "trailers", "content-type": "application/grpc", "user-agent": "tonic/0.10.2"} }, message: HelloRequest { name: "Tonic" }, extensions: Extensions }
```

The client will show:

```
RESPONSE=Response { metadata: MetadataMap { headers: {"content-type": "application/grpc", "date": "Tue, 30 Jan 2024 19:56:42 GMT", "grpc-status": "0"} }, message: HelloReply { message: "Hello Tonic!" }, extensions: Extensions }
```

It's a little more involved than just building a REST server, but it's fast, efficient---and you can quickly generate clients for many languages.