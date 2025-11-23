use crate::entities::line::Line;
use axum::http::HeaderMap;
use reqwest::Client;
use serde_json::{Map, json};
use std::num::ParseIntError;

/// Returns the necessary headers to make calls to the STAN API
/// 
/// # Panics
/// Should never panic; it calls .unwrap on a static operation that either never fails 
/// or always fails
#[must_use]
pub fn get_stan_api_calls_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        "application/x-www-form-urlencoded; charset=UTF-8"
            .parse()
            .unwrap(),
    );
    headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());

    headers
}

/// Function that fetches a presigned URL for the URL passed as parameter.
///
/// Navitia requests work this way:
/// - Client needs to get information from the Navitia API
/// - Client generates an x-auth-token and asks for a presigned URL
///   to <https://nws-main.hove.io/api/presign>
/// - Navitia API returns a json object with a field `url` that corresponds to the presigned URL
/// - Client can then ask the API for information to <https://nws-main.hove.io/api/proxy>,
///   using the presigned URL in its JSON body and the x-auth-token as a header
///
/// # Errors
/// Returns `anyhow::Error` if the request failed
pub async fn request_presigned_navitia_url(
    client: &Client,
    url: String,
    x_auth_token: &str,
) -> anyhow::Result<String> {
    let json_response: Map<String, serde_json::Value> = client
        .post("https://nws-main.hove.io/api/presign")
        .header("origin", "https://nws-main.hove.io")
        .header("X-Auth", x_auth_token)
        .json(&json!({
            "method": "POST",
            "path": url,
            "clientName": "stan"
        }))
        .send()
        .await?
        .json()
        .await?;

    json_response.get("url").map_or_else(
        || Err(anyhow::anyhow!("Could not get the presigned URL")),
        |v| {
            v.as_str().map_or_else(
                || {
                    Err(anyhow::anyhow!(
                        "URL field missing from the presigned URL response body"
                    ))
                },
                |v| Ok(v.to_string()),
            )
        },
    )
}

#[must_use]
pub fn get_line_from_attribute(line_attribute: &str, all_lines: &[Line]) -> Option<Line> {
    // If it starts with line:GST: -> line.id
    if line_attribute.starts_with("line:GST:") {
        return all_lines.iter().find(|l| l.id == line_attribute).cloned();
    }

    // If it parses into a number -> line.number
    let parsed: Result<usize, ParseIntError> = line_attribute.parse();

    parsed.map_or_else(
        |_| all_lines.iter().find(|l| l.code == line_attribute).cloned(),
        |n| all_lines.iter().find(|l| l.number == n).cloned(),
    )
}

#[must_use]
pub fn get_style_value_from_elt(style: &str, key: &str) -> Option<String> {
    style.split(';').find_map(|rule| {
        let mut parts = rule.split(':');
        let k = parts.next()?.trim();
        let v = parts.next()?.trim();
        if k == key { Some(v.to_string()) } else { None }
    })
}
