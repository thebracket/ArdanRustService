# Hello Tonic - Project Definition

We're going to take advantage of a Rust trick to define two binaries in one project. That way, inside one project we can build both the server and the client---and output two different binaries.

You don't *strictly* need to do this, but it's convenient for keeping the code together. It also lets you learn a handy Rust trick!

Edit your project's `Cargo.toml`:

```toml
[package]
name = "hello_tonic"
version = "0.1.0"
edition = "2021"

[[bin]] # Bin to run the HelloWorld gRPC server
name = "helloworld-server"
path = "src/server.rs"

[[bin]] # Bin to run the HelloWorld gRPC client
name = "helloworld-client"
path = "src/client.rs"

[dependencies]
tonic = "0.10"
prost = "0.12"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }

[build-dependencies]
tonic-build = "0.10"
```

You've specified multiple `[[bin]]` sections, allowing you to name programs and replace `main.rs` with named files for each program.

You've also specified some dependencies:

* `tonic` is the gRPC server/client itself.
* `prost` is a Protobuf compiler that converts `proto` files to Rust.
* `tokio` provides the underlying async setup.

You've also got a *build-dependency*---it won't be in the main program, but will be required when you build your program.

## Build.rs

Now let's add a `build.rs` file. It goes in the same folder as your `Cargo.toml` file - **not in src**. `build.rs` is a special file that is invoked before compilation occurs, you can use it to perform pre-build tasks.

Our `build.rs` file is quite simple:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/hello.proto")?;
    Ok(())
}
```

The file calls `tonic_build` and tells it to compile our protobuf file.

If you build your project now, deep inside your `target` directory the protobuf file is compiled into a `hello.rs` file. It's quite intimidating. Here's just a snippet:

```rust
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloReply {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod greeter_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct GreeterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GreeterClient<tonic::transport::Channel> {
```

Prost and the builder have implemented an entire Rust system! Your messages are implemented as Rust structures, connected to the Tonic system. If you keep scrolling, there's a complete async reactor service in there! Good thing you didn't have to type that...