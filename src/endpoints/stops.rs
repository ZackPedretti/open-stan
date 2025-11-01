use crate::entities::api_state::ApiState;
use crate::entities::lane::Lane;
use crate::entities::stop::Stop;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use reqwest::header::HeaderMap;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::fmt::format;

pub fn router() -> Router<ApiState> {
    Router::new().route("/", get(get_stops_of_lane))
}
async fn get_stops_of_lane(
    Query(query): Query<StopQuery>,
    State(api_state): State<ApiState>,
) -> impl IntoResponse {
    match Lane::from_text(&*query.lane) {
        None => (
            StatusCode::NOT_FOUND,
            format!("'{}': Lane not found", query.lane),
        )
            .into_response(),
        Some(l) => fetch_stops_of_lane(api_state.client, l)
            .await
            .into_response(),
    }
}

fn convert_response_into_json(res: String) -> anyhow::Result<Json<Vec<Stop>>> {
    println!("{}", res);
    todo!()
}

async fn fetch_stops_of_lane(client: Client, lane: Lane) -> Json<Vec<Stop>> {
    convert_response_into_json(request_stops_of_lane(client, lane).await.unwrap())
        .expect("TODO: panic message");
    todo!()
}

async fn request_stops_of_lane(client: Client, lane: Lane) -> reqwest::Result<String> {
    client
        .post("https://www.reseau-stan.com/?type=476")
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:144.0) Gecko/20100101 Firefox/144.0")
        .header("Accept", "*/*")
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Accept-Encoding", "gzip, deflate, br, zstd")
        .header("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8")
        .header("X-Requested-With", "XMLHttpRequest")
        .header("Content-Length", "158")
        .header("Origin", "https://www.reseau-stan.com")
        .header("Sec-GPC", "1")
        .header("Connection", "keep-alive")
        .header("Referer", "https://www.reseau-stan.com/")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-origin")
        .header("Priority", "u=0")
        .header("TE", "trailers")
        .body(format!("requete=tempsreel_arrets&requete_val%5Bligne%5D=2484&requete_val%5Bcolor%5D=%23FFFFFF&requete_val%5Bbackground%5D=%23E30613&requete_val%5Bnumlignepublic%5D={}", lane.to_text()))
        .send()
        .await?
        .text()
        .await
}

#[derive(Deserialize)]
struct StopQuery {
    lane: String,
}
