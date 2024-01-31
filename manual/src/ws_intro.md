# Web Sockets

Sometimes you want a connection that's more like a regular TCP connection. As you saw in Rust Foundations, this can offer significant performance and bandwidth advantages. It's also a little irritating to write in that you are building a complete protocol from scratch.

gRPC offers a good middle-ground: you reuse your connection, the protocol is largely implemented for you, and it's relatively painless.

Sometimes, though, you need to stream a large amount of data to a client and would like a bit more control over the protocol. This is where WebSockets come in handy, especially for large volumes of data to be displayed in a browser.

Axum has WebSocket support baked in, but it's quite unlike regular REST requests.