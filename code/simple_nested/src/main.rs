use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {

    let app = Router::new()
        .nest("/1", service_one())
        .nest("/2", service_two());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}


fn service_one() -> Router {
    Router::new().route("/", get(|| async { 
        Html("Service One".to_string()) }))
}

fn service_two() -> Router {
    Router::new().route("/", get(|| async { 
        Html("Service Two".to_string()) }))
}