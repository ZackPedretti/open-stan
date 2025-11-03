use crate::entities::{api_state::ApiState, line::Line};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::{Map, json};

pub fn router() -> Router<ApiState> {
    Router::new().route("/", get(get_all_lanes))
}

async fn get_all_lanes(State(state): State<ApiState>) -> impl IntoResponse {
    let presigned_url =
        match request_presigned(&state, "/v1/coverage/fr-ne-nancy/lines".to_string()).await {
            Ok(v) => v,
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        };

    println!("{}", &presigned_url);

    match request_lines(&state, presigned_url).await {
        Ok(v) => Json(v).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn request_presigned(state: &ApiState, url: String) -> anyhow::Result<String> {
    let json_response: Map<String, serde_json::Value> = state
        .client
        .post("https://nws-main.hove.io/api/presign")
        .json(&json!({
            "method": "POST",
            "path": url,
            "query": {
                "disable_geojson": "true",
                "count": "100",
                "filter": "physical_mode.id=physical_mode:Bus"
            },
            "clientName": "stan"
        }))
        .send()
        .await?
        .json()
        .await?;

    match json_response.get("url") {
        None => Err(anyhow::anyhow!("Could not get the presigned URL")),
        Some(v) => Ok(v.as_str().unwrap().to_string()),
    }
}

#[derive(Deserialize)]
struct LinesResponse {
    lines: Vec<Line>,
}

async fn request_lines(state: &ApiState, presigned_url: String) -> anyhow::Result<Vec<Line>> {
    let json_response: LinesResponse = state
        .client
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
