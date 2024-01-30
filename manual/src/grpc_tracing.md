# Tracing

Everything you know about Tracing still applies! Include `tracing` in your server or client, and start emitting log entries and spans. Include `tracing_subscriber` to output your results.

Let's modify our hello world server to include some tracing.

> The code for this is in `code/grpc/tonic_tracing`

We'll need to add some dependencies:

```bash
cargo add tracing
cargo add tracing-subscriber
```

Now in `server.rs`, we can add a timing span to our function:

```rust
#[tracing::instrument]
    async fn say_hello(
```

And in `main`, we need to activate tracing. I've forced it to `DEBUG` level to show a LOT of messages:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup tracing
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Include per-span timings
        .with_span_events(FmtSpan::CLOSE)
        // Build the subscriber
        .finish();

    // Set the subscriber as the default
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .trace_fn(|_| tracing::info_span!("hello_server"))
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
```
