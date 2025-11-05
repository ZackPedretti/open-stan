use crate::entities::{api_state::ApiState, line::Line};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use serde_json::{Map, json};
use crate::utils::request_presigned;

pub fn router() -> Router<ApiState> {
    Router::new().route("/", get(get_all_lanes))
}

async fn get_all_lanes(State(state): State<ApiState>) -> impl IntoResponse {
    let presigned_url =
        match request_presigned(&state.client, "/v1/coverage/fr-ne-nancy/lines".to_string()).await {
            Ok(v) => v,
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        };

    match request_lines(&state.client, presigned_url).await {
        Ok(v) => Json(v).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(Deserialize)]
struct LinesResponse {
    lines: Vec<Line>,
}

pub async fn request_lines(client: &Client, presigned_url: String) -> anyhow::Result<Vec<Line>> {
    let json_response: LinesResponse = client
        .post("https://nws-main.hove.io/api/proxy")
        .json(&json!({
            "presignedUrl": presigned_url,
            "href": "https://api.navitia.io/v1/coverage/fr-ne-nancy/lines",
            "clientName": "stan"
        }))
        .send()
        .await?
        .json()
        .await?;

    Ok(json_response.lines)
}
