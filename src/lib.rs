#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

use crate::entities::api_doc::ApiDoc;
use crate::entities::api_state::ApiState;
use axum::Router;
use axum::routing::get;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod endpoints;
pub mod entities;
pub mod navitia_token;
pub mod utils;

pub async fn welcome() -> &'static str {
    "Hello, world!"
}

/// Creates a rooter for Axum with the correct client configuration, endpoints and state
///
/// # Errors
/// Returns an `anyhow::Error` if the Reqwest client could not be built successfully
pub fn init_router() -> anyhow::Result<Router> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:144.0) Gecko/20100101 Firefox/144.0")
        .build()?;
    let state = ApiState { client };
    let router = Router::new()
        .route("/", get(welcome))
        .nest("/lines", endpoints::lines::router())
        .nest("/stops", endpoints::stops::router())
        .nest("/arrivals", endpoints::arrivals::router())
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()));
    Ok(router.with_state(state))
}
