use std::sync::Arc;

use axum::{response::Html, routing::get, Router, extract::State};

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

struct MyState {
    n: i32,
}

fn service_one() -> Router {
    let state = Arc::new(MyState { n: 1 });
    Router::new()
        .route("/", get(handler))
        .with_state(state)
}

fn service_two() -> Router {
    let state = Arc::new(MyState { n: 2 });
    Router::new()
        .route("/", get(handler))
        .with_state(state)
}

async fn handler(State(my_state): State<Arc<MyState>>) -> Html<String> {
    Html(format!("State: {}", my_state.n))
}