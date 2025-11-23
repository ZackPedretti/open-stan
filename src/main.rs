use open_stan::init_router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = init_router().expect("There was an issue while building the app Router.");

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
