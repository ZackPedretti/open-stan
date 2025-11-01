use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::get;
use crate::entities::lanes::Lane;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_all_lanes))
}

async fn get_all_lanes() -> impl IntoResponse {
    Json(Lane::all())
}