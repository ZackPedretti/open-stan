use axum::Router;
use open_stan::{endpoints, entities::ApiState};

pub fn init_router() -> Router<ApiState> {
    Router::new()
        .nest("/lines", endpoints::lines::router())
        .nest("/stops", endpoints::stops::router())
        .nest("/arrivals", endpoints::arrivals::router())
}
