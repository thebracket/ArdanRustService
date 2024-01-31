# Minimal Echo Server

Let's create a project:

```bash
cargo new ws_echo
cd ws_echo
cargo add axum -F ws
cargo add tokio -F full
```

And here's a working server:

> The code is in `code/wss/wss_echo`.

```rust
use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/", get(site));

        let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn site() -> Html<&'static str> {
    const BODY: &str = include_str!("echo.html");
    Html(BODY)
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |sock| handle_socket(sock))
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        println!("Got message: {}", msg.to_text().unwrap());
        socket.send(msg).await.unwrap();
    }
}
```

Let's go through the functions:

* `main` is a typical async main function, and sets up an Axum router as we have before.
* `site` returns a static HTML page from `echo.html`. The page is compiled into the binary using `include_str`.

The two websocket functions as `ws_handler` and `handle_socket`. Browser-based websockets are opened with a small dance:

1. The browser gets the web-socket url (`ws://localhost:3001/ws`) in this case.
2. The server accepts the connection and offers websocket negotiation.
3. The browser opens a websocket with an "upgrade" call.

The `ws_handler` function receives the request for a websocket, and binds a function to `on_upgrade` - when the client upgrades to an actual websocket connection, an async task will be spawned with the passed function.

So `ws_handler` *indirectly* calls `handle_socket`---it won't activate until the socket is running.

Inside `handle_socket`, you can see how websockets work from the server perspective. A websocket isn't quite a raw TCP connection---it automatically wraps each transmission in a "message" (so there's no need to handle size negotiations in your protocol). This allows Axum to send you messages as a stream. When the socket closes, the stream closes - and the function's `while let` will stop. In this case, the function then stops.

This is a *really* powerful and fast mechanism. You can have gRPC style streaming while just using a webserver/browser.

Here's the HTML page:

```html
<html>
<head>
<title>WebSocket Echo Client</title>
</head>
<body>
    <input type="text" id="message" value="Hello, world!">
    <input type="button" id="send" value="Send">

    <script>
        var ws = new WebSocket("ws://localhost:3001/ws");
        ws.onopen = function() {
            console.log("onopen");
        };
        ws.onmessage = function(e) {
            console.log("onmessage: " + e.data);
        };
        ws.onclose = function() {
            console.log("onclose");
        };
        ws.onerror = function(e) {
            console.log("onerror: " + e.data);
        };

        document.getElementById("send").onclick = function() {
            var message = document.getElementById("message").value;
            ws.send(message);
        };
    </script>
</body>
</html>
```

Try this out: go to `http://localhost:3001`. Open the debug console, and you can see an "onopen" message indicating that the socket opened. Submit some text, and it will be echoed back.