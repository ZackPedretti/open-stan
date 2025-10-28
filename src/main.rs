async fn hello_world() -> &'static str {
    "Hello, world!"
}

use axum::{Router, routing::get};
use std::net::SocketAddr;

fn init_router() -> Router {
    Router::new().route("/", get(hello_world))
}

#[tokio::main]
async fn main() {
    let app = init_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
