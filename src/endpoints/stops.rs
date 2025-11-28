use crate::endpoints::lines::request_lines;
use crate::entities::ApiState;
use crate::entities::Stop;
use crate::entities::api_query_args::GetStopOfLineQueryArgs;
use crate::navitia_token::create_token;
use crate::utils::{get_line_from_attribute, request_presigned_navitia_url};
use anyhow::anyhow;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashSet;

pub fn router() -> Router<ApiState> {
    let router: Router<ApiState> = Router::new().route("/", get(get_stops));
    router
}

#[utoipa::path(
    get,
    path = "/stops",
    params(
        ("line" = Option<String>, Query, 
         description = "Optional line identifier (ID, number, or code). If provided, returns stops only for this line; otherwise, returns all stops.")
    ),
    responses(
        (status = 200, description = "List of stops", body = [Stop]),
    ),
    tag = "stops",
    description = "Retrieve bus stops. Supports optional filtering by line."
)]

pub async fn get_stops(
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

async fn get_stops_of_line(
    line: String,
    client: &Client,
) -> anyhow::Result<Vec<Stop>> {
    let x_auth_token = create_token();
    let presigned_url = request_presigned_stops_of_line(client, &line, &x_auth_token).await?;
    let all_lines = request_lines(client).await?;
    let line = get_line_from_attribute(&line, &all_lines);
    match line {
        None => Err(anyhow!("Invalid line argument")),
        Some(line) => request_stops_of_line(line.id, presigned_url, client, &x_auth_token).await,
    }
}

#[derive(Deserialize, Debug)]
struct StopsResponse {
    stop_areas: Vec<Stop>,
}

async fn get_all_stops(client: &Client) -> anyhow::Result<Vec<Stop>> {
    let lines = request_lines(client).await?;
    let x_auth_token = create_token();
    let mut all_stops: HashSet<Stop> = HashSet::new();

    for line in lines {
        let presigned_url = request_presigned_stops_of_line(client, &line.id, &x_auth_token).await?;
        let stops_of_line = request_stops_of_line(line.id, presigned_url, client, &x_auth_token).await?;
        for stop in stops_of_line {
            all_stops.insert(stop);
        }
    }

    Ok(all_stops.into_iter().collect())
}

async fn request_presigned_stops_of_line(
    client: &Client,
    line_id: &str,
    x_auth_token: &str,
) -> anyhow::Result<String> {
    request_presigned_navitia_url(
        client,
        format!("/v1/coverage/fr-ne-nancy/lines/{line_id}/stop_areas"),
        x_auth_token,
    )
    .await
}

async fn request_stops_of_line(
    line_id: String,
    presigned_url: String,
    client: &Client,
    x_auth_token: &str,
) -> anyhow::Result<Vec<Stop>> {
    let url = format!("https://api.navitia.io/v1/coverage/fr-ne-nancy/lines/{line_id}/stop_areas?count=100&depth=3");

    let json_response: reqwest::Result<StopsResponse> = client
        .post("https://nws-main.hove.io/api/proxy")
        .header("origin", "https://nws-main.hove.io")
        .header("x-auth-token", x_auth_token)
        .json(&json!({
            "presignedUrl": presigned_url,
            "href": &url,
            "clientName": "stan"
        }))
        .send()
        .await?
        .json()
        .await;

    match json_response {
        Ok(r) => Ok(r.stop_areas),
        Err(e) => {
            println!("{}", e.to_string());
            Err(anyhow::anyhow!(
                "Was unable to parse JSON response. Could be due to an error returned by the Navitia API."
            ))
        }
    }
}
