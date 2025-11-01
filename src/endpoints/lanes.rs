use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::get;
use crate::entities::api_state::ApiState;
use crate::entities::lane::Lane;

pub fn router() -> Router<ApiState> {
    Router::new()
        .route("/", get(get_all_lanes))
}

async fn get_all_lanes() -> impl IntoResponse {
    Json(Lane::all())
}