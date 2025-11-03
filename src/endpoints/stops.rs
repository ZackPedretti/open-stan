use axum::Router;

use crate::entities::api_state::ApiState;

pub fn router() -> Router<ApiState> {
    let router: Router<ApiState> = Router::new();
    router
}
