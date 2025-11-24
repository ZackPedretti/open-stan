use crate::entities::{api_state::ApiState, line::Line};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use reqwest::{Client, StatusCode};
use scraper::{Html, Selector};

pub const STAN_API_LINES_URL: &str = "https://www.reseau-stan.com/";

pub fn router() -> Router<ApiState> {
    Router::new().route("/", get(get_lines))
}

#[utoipa::path(get, path = "/lines")]
pub async fn get_lines(State(state): State<ApiState>) -> impl IntoResponse {
    match request_lines(&state.client, STAN_API_LINES_URL).await {
        Ok(v) => Json(v).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/// Fetches the bus lines from the official STAN website (<https://www.reseau-stan.com/>), parses the HTML,
/// and converts the information into `Line` objects.
///
/// # Errors
/// Returns an `anyhow::Error` if an error happened during requesting or parsing the HTML
pub async fn request_lines(client: &Client, url: &str) -> anyhow::Result<Vec<Line>> {
    let html = client.get(url).send().await?.text().await?;

    parse_document_into_lines(&html)
}

fn parse_document_into_lines(html: &str) -> anyhow::Result<Vec<Line>> {
    let mut lines: Vec<Line> = vec![];
    let line_options_selector = match Selector::parse("select#form_ligne option") {
        Ok(s) => s,
        Err(err) => {
            return Err(anyhow::anyhow!(err.to_string()));
        }
    };
    
    let document = Html::parse_document(&html);

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
            color: get("data-background"),
            text_color: get("data-color"),
        };

        lines.push(line);
    }

    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_document_into_lines_test() {
        let html = std::fs::read_to_string("tests/data/homepage.html").unwrap();
        let parsed_result = parse_document_into_lines(&html);
        assert!(parsed_result.is_ok());
        assert!(parsed_result.unwrap().len() > 0);
    }
}