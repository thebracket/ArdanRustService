# Hello Tonic - The Server

Rename your `main.rs` file to `server.rs`. This is where we'll build the server.

Start with some imports:

```rust
use tonic::{transport::Server, Request, Response, Status};
```

Next, we'll create a module and include the entire auto-generated protocol file in it:

```rust
pub mod hello_world {
    tonic::include_proto!("hello");
}
```

All of the code you didn't have to write is now in `hello_world::<foo>`! We need a few of the defined functions, so let's `use` them:

```rust
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
```

We're importing the `Greeter` service and server, and the message types we defined. Tonic does a bit of trait magic, providing your server definitions as traits. So we have to implement our own structure and implement a trait to make our server do something:

```rust
#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}
```

In this case, we print out that we received a request, format a reply and send it right back. The last thing to do is write a `main` function and start the server:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
```