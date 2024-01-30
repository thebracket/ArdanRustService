# gRPC Streaming

REST requests tend to be distinct: you send a request and get a response. You *can* implement streaming in HTTP, but it tends to get complicated.

gRPC lets you abstract a *stream* over a reusable client, and interact with it like you would a Rust async channel. If you think back to *Rust Foundations*, you can treat streams like an iterator: progressively sending more and more data until the task is done. When the stream is done, the channel closes---and the recipient doesn't receive any more messages.

Let's make a new project:

```bash
cargo new tonic_stream
cd tonic_stream
```

And edit our `Cargo.toml` to include both a client and server, and some dependencies. We've added one more: `tokio_stream`:

```toml
[package]
name = "tonic_stream"
version = "0.1.0"
edition = "2021"

[[bin]] # Bin to run the HelloWorld gRPC server
name = "streaming-server"
path = "src/server.rs"

[[bin]] # Bin to run the HelloWorld gRPC client
name = "streaming-client"
path = "src/client.rs"

[dependencies]
tonic = "0.10"
prost = "0.12"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
tokio-stream = "0.1.14"

[build-dependencies]
tonic-build = "0.10"
```