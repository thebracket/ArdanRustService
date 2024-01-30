# Authentication

Some authentication systems are built-in to the gRPC standard, and may be used through Tonic. And just like Axum, you can add layers to handle authentication.

Let's go back to our `hello world` example and add some authentication.

> The code for this is in `code/grpc/tonic_auth`.

We can add server-side authentication by adding a function:

```rust
fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    use tonic::metadata::MetadataValue;
    let token: MetadataValue<_> = "Bearer some-secret-token".parse().unwrap();

    match req.metadata().get("authorization") {
        Some(t) if token == t => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}
```

So we're requiring that the client provide some meta-data containing a secret token. If it isn't present, we'll return an error and a status code.

We also need to adjust our service main to use it:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    let svc = GreeterServer::with_interceptor(greeter, check_auth);

    Server::builder()
        .add_service(svc)
        .serve(addr)
        .await?;

    Ok(())
}
```

We've added an *interceptor*---which receives requests before the actual handler. This is just like using layers in Axum/Tower.

Now run the client unmodified:

1. Run the server with `cargo run --bin auth-server`
2. Run the client with `cargo run --bin auth-client`

You will receive an error code:

```
Error: Status { code: Unauthenticated, message: "No valid auth token", metadata: MetadataMap { headers: {"content-type": "application/grpc", "date": "Tue, 30 Jan 2024 21:25:27 GMT", "content-length": "0"} }, source: None }
```

That's great if all you want to do is deny requests! How do we have the client perform authentication?

Here's the modified client:

```rust
pub mod hello_world {
    tonic::include_proto!("tonic_auth");
}

use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use tonic::Request;
use crate::hello_world::greeter_client::GreeterClient;
use crate::hello_world::HelloRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the client as a channel with a token and interceptor
    let channel = Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;
    let token: MetadataValue<_> = "Bearer some-secret-token".parse()?;
    let mut client = GreeterClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

    // Using the client remains unchanged
    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
```

We've replaced `let mut client = GreeterClient::connect("http://[::1]:50051").await?;` with a more complicated connection scheme:

* We create a channel to represent the connection.
* We create the token.
* We create the `GreeterClient` with an interceptor that adds the authentication to the request.

Running the client now succeeds.