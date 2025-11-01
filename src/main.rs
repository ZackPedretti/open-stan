mod endpoints;
mod entities;

async fn welcome() -> &'static str {
    "Hello, world!"
}

use axum::{Router, routing::get};
use std::net::SocketAddr;
use reqwest::Client;
use crate::entities::api_state::ApiState;

fn init_router() -> Router{
    let state = ApiState { client: Client::new() };
    let router = Router::new()
        .route("/", get(welcome))
        .nest("/lanes", endpoints::lanes::router())
        .nest("/stop", endpoints::stops::router());
    router.with_state(state)
}

#[tokio::main]
async fn main() {
    let app = init_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
