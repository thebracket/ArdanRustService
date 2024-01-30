use std::collections::HashMap;

use axum::{response::Html, routing::get, Router};
use axum::extract::Query;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/book", get(query_extract));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn query_extract(
    Query(params): Query<HashMap<String, String>>,
) -> Html<String>
{
    Html(format!("{:#?}", params))
}