use crate::entities::{api_state::ApiState, line::Line};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use reqwest::{Client, StatusCode};
use scraper::{Html, Selector};

pub fn router() -> Router<ApiState> {
    Router::new().route("/", get(get_lines))
}

#[utoipa::path(get, path = "/lines")]
pub async fn get_lines(State(state): State<ApiState>) -> impl IntoResponse {
    match request_lines(&state.client).await {
        Ok(v) => Json(v).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/// Fetches the bus lines from the official STAN website (<https://www.reseau-stan.com/>), parses the HTML,
/// and converts the information into `Line` objects.
///
/// # Errors
/// Returns an `anyhow::Error` if an error happened during requesting or parsing the HTML
pub async fn request_lines(client: &Client) -> anyhow::Result<Vec<Line>> {
    let html = client
        .get("https://www.reseau-stan.com/")
        .send()
        .await?
        .text()
        .await?;

    let mut lines: Vec<Line> = vec![];

    let document = Html::parse_document(&html);
    let line_options_selector = match Selector::parse("select#form_ligne option") {
        Ok(s) => s,
        Err(err) => {
            return Err(anyhow::anyhow!(err.to_string()));
        }
    };

    for elt in document.select(&line_options_selector) {
        if elt.value().attr("disabled").is_some() {
            continue;
        }

        let get = |name: &str| elt.value().attr(name).unwrap_or("").to_string();

        let line = Line {
            id: get("data-osmid"),
            number: get("value").parse()?,
            name: get("data-libelle"),
            code: get("data-numlignepublic"),
            color: get("data-backgroundcolor"),
            text_color: get("data-color"),
        };

        lines.push(line);
    }

    Ok(lines)
}
