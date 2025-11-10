mod endpoints;
mod entities;
mod utils;

async fn welcome() -> &'static str {
    "Hello, world!"
}

use crate::entities::api_state::ApiState;
use axum::{Router, routing::get};
use std::net::SocketAddr;

fn init_router() -> anyhow::Result<Router> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:144.0) Gecko/20100101 Firefox/144.0")
        .build()?;
    let state = ApiState { client };
    let router = Router::new()
        .route("/", get(welcome))
        .nest("/lines", endpoints::lines::router())
        .nest("/stops", endpoints::stops::router())
        .nest("/times", endpoints::arrivals::router());
    Ok(router.with_state(state))
}

#[tokio::main]
async fn main() {
    let app = init_router().expect("There was an issue while building the app Router.");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
