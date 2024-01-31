# JSON

We've sent and received plain text, which is often enough. But what if we want to stream some structured data in JSON, for example to update some telemetry at a high rate?

The code for this is in `code/wss/ws_json`.

Let's start by creating a project and dependencies:

```bash
cargo new ws_json
cd ws_json
cargo add tokio -F full
cargo add axum -F ws
cargo add serde -F derive
cargo add serde_json
```

Now we'll build a server. It's very similar to the echo example, but we've added strongly typed JSON:

```rust
use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
enum Request {
    NextPoint,
    Quit
}

#[derive(Serialize, Deserialize)]
struct Point {
    x: f32,
    y: f32,
}

async fn site() -> Html<&'static str> {
    const BODY: &str = include_str!("json.html");
    Html(BODY)
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |sock| handle_socket(sock))
}

async fn handle_socket(mut socket: WebSocket) {
    let mut tick = 0.0f32;
    while let Some(Ok(msg)) = socket.recv().await {
        let text = msg.to_text().unwrap();
        let req: Request = serde_json::from_str(text).unwrap();
        match req {
            Request::NextPoint => {
                let response = Point {
                    x: tick.cos() * (tick / 10.0),
                    y: tick.sin() * (tick / 10.0),
                };
                let response = serde_json::to_string(&response).unwrap();
                socket.send(response.into()).await.unwrap();

                tick += 0.01;
                if tick > 4_000.0 {
                    tick = 0.0;
                }
            }
            Request::Quit => {
                break;
            }
        }
    }
}
```

Notice how we:

* Use Serde's `Serialize` and `Deserialize` to make our two data-types serializable.
* Setup a request/response format.
* When we receive a message, we deserialize it ourselves.
* We maintain state within the `handle_socket` function. Each function runs independently---so the state is *local* to a connection. If we connect twice, each will run the full cycle.
* Quitting is as simple as breaking out of the handler loop, which will drop the socket.

The client side (`json.html`) is a bit complicated, so let's cut and paste it:

```html
<html>
<head>
<title>WebSocket Echo Client</title>
</head>
<body>
    <canvas id="myCanvas" width="600" height="600"></canvas>
    <div id="counter"></div>

    <script>
        const canvas = document.getElementById('myCanvas');
        const ctx = canvas.getContext("2d");
        const requestNext = { NextPoint : null };
        const start = Date.now();
        let red = 0;
        let iterations = 0;

        var ws = new WebSocket("ws://localhost:3001/ws");
        ws.onopen = function() {
            console.log("onopen");
            ws.send(JSON.stringify(requestNext));
        };
        ws.onmessage = function(e) {
            let message = JSON.parse(e.data);
            // Move to the center
            message.x += 300;
            message.y += 300;
            ctx.fillStyle = "rgb(" + red + ", 0, 0)";
            red += 1;
            red %= 255;
            ctx.fillRect(message.x, message.y, 1, 1);
            iterations += 1;
            document.getElementById("counter").innerHTML = iterations;

            if (iterations < 200000) {
                ws.send(JSON.stringify(requestNext));
            } else {
                const end = Date.now();
                const time = end - start;
                const speed = iterations / time;
                document.getElementById("counter").innerHTML = "Iterations: " + iterations + " Time: " + time + " Speed: " + speed.toFixed(2) + " points/ms";
                ws.send(JSON.stringify({ Quit : null }));
            }
        };
        ws.onclose = function() {
            console.log("onclose");
        };
        ws.onerror = function(e) {
            console.log("onerror: " + e.data);
        };
    </script>
</body>
</html>
```

Go to [http://localhost:3001](http://localhost:3001) and see. Great performance! And more importantly, all 200,000 requests have used a single TCP stream---and a single header. Once the socket is running, it's very low overhead.