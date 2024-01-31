use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut ws_stream, _) = tokio_tungstenite::connect_async("ws://localhost:3001/ws").await?;
    let message = Message::Text("Hello".to_string());
    ws_stream.send(message).await?;

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        println!("Received: {}", msg);
    }

    Ok(())
}
