# gRPC Streaming - Protocol Definition

Next, we define the protocol again:

```bash
mkdir proto
touch streaming.proto
```

And we fill in the protocol:

```proto
syntax = "proto3";

package streaming;

service streaming {
    rpc Squares(Start) returns (stream Square);
}

message Start {
    int32 n = 1;
}

message Square {
    int32 n = 1;
}
```

Notice that we're returning `stream Square`---we're going to stream the results.

Let's also put our `build.rs` in place to compile the protocol:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/streaming.proto")?;
    Ok(())
}
```