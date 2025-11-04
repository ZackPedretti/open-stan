use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::get;
use reqwest::{Client, StatusCode};
use scraper::{Html, Selector};
use serde::Deserialize;
use serde_json::json;
use crate::entities::api_state::ApiState;
use crate::entities::stop::Stop;
use crate::utils::{get_stan_api_calls_headers, request_presigned};

pub fn router() -> Router<ApiState> {
    let router: Router<ApiState> = Router::new()
        .route("/", get(get_stops_of_line));
    router
}

#[derive(Deserialize)]
struct GetStopOfLineQueryArgs {
    line: String,
}

async fn get_stops_of_line(Query(query): Query<GetStopOfLineQueryArgs>, State(state): State<ApiState>) -> impl IntoResponse {

    let presigned_url = match request_presigned(&state, format!("/v1/coverage/fr-ne-nancy/lines/{}/stop_areas", &query.line)).await {
        Ok(v) => v,
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    };

    match request_stops_of_line(query.line, presigned_url, &state.client).await {
        Ok(v) => Json(v).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response()
    }
}

#[derive(Deserialize, Debug)]
struct StopsResponse {
    stop_areas: Vec<Stop>,
}

async fn request_stops_of_line(line_id: String, presigned_url: String, client: &Client) -> anyhow::Result<Vec<Stop>> {
    let url = format!("https://api.navitia.io/v1/coverage/fr-ne-nancy/lines/{}/stop_areas?count=100&depth=3", line_id);

    let json_response: StopsResponse = client
        .post("https://nws-main.hove.io/api/proxy")
        .json(&json!({
            "presignedUrl": presigned_url,
            "href": &url,
            "clientName": "stan"
        }))
        .send()
        .await?
        .json()
        .await?;

    Ok(json_response.stop_areas)
}