use crate::entities::api_state::ApiState;
use crate::entities::line::Line;
use crate::entities::stop::Stop;
use crate::entities::time::Time;
use crate::utils::get_stan_api_calls_headers;
use axum::Router;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::routing::get;
use reqwest::{Client, StatusCode};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

pub fn router() -> Router<ApiState> {
    let router: Router<ApiState> = Router::new().route("/", get(get_remaining_time_to_stop));

    router
}

#[derive(Deserialize, Serialize, Debug)]
struct GetRemainingTimeToStopQueryArgs {
    stop: String,
    line: Option<String>,
}

async fn get_remaining_time_to_stop(
    Query(query): Query<GetRemainingTimeToStopQueryArgs>,
    State(state): State<ApiState>,
) -> impl IntoResponse {
    StatusCode::OK.into_response()
}

async fn request_remaining_time_to_stop(
    stop: String,
    line: Option<String>,
    client: &Client,
    lines: Vec<Line>,
) -> anyhow::Result<Vec<Time>> {
    let html_text = client
        .post("https://www.reseau-stan.com/?type=476")
        .headers(get_stan_api_calls_headers())
        .body(format!(
            "requete=tempsreel_submit&requete_val%5Barret%5D=stop_point%3AGST%3ASP%3A{}0",
            Stop::get_unique_identifier_from_str(&stop)
        ))
        .send()
        .await?
        .text()
        .await?;

    todo!()
}
