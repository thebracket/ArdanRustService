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
