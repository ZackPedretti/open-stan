use crate::endpoints::lines::request_lines;
use crate::entities::api_state::ApiState;
use crate::entities::stop::Stop;
use crate::utils::{request_presigned};
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashSet;
use crate::entities::api_query_args::GetStopOfLineQueryArgs;

pub fn router() -> Router<ApiState> {
    let router: Router<ApiState> = Router::new().route("/", get(get_stops));
    router
}

async fn get_stops(
    Query(query): Query<GetStopOfLineQueryArgs>,
    State(state): State<ApiState>,
) -> impl IntoResponse {
    let stops = match query.line {
        None => get_all_stops(&state.client).await,
        Some(l) => get_stops_of_line(l, &state.client).await,
    };

    match stops {
        Ok(v) => Json(v).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn get_stops_of_line(line: String, client: &Client) -> anyhow::Result<Vec<Stop>> {
    let presigned_url = request_presigned_stops_of_line(client, &line).await?;

    Ok(request_stops_of_line(line, presigned_url, client).await?)
}

#[derive(Deserialize, Debug)]
struct StopsResponse {
    stop_areas: Vec<Stop>,
}

async fn get_all_stops(client: &Client) -> anyhow::Result<Vec<Stop>> {
    let lines = request_lines(client).await?;

    let mut all_stops: HashSet<Stop> = HashSet::new();

    for line in lines {
        let presigned_url = request_presigned_stops_of_line(client, &line.id).await?;
        let stops_of_line = request_stops_of_line(line.id, presigned_url, client).await?;
        for stop in stops_of_line {
            all_stops.insert(stop);
        }
    }

    Ok(all_stops.into_iter().collect())
}

async fn request_presigned_stops_of_line(
    client: &Client,
    line_id: &String,
) -> anyhow::Result<String> {
    request_presigned(
        client,
        format!("/v1/coverage/fr-ne-nancy/lines/{}/stop_areas", line_id),
    )
    .await
}

async fn request_stops_of_line(
    line_id: String,
    presigned_url: String,
    client: &Client,
) -> anyhow::Result<Vec<Stop>> {
    let url = format!(
        "https://api.navitia.io/v1/coverage/fr-ne-nancy/lines/{}/stop_areas?count=100&depth=3",
        line_id
    );

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
