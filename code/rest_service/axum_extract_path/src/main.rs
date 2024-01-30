use axum::{response::Html, routing::get, Router};
use axum::extract::Path;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/book/:id", get(path_extract));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn path_extract(
    Path(id): Path<u32>,
) -> Html<String>
{
    Html(format!("Hello, {}!", id))
}