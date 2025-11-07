use std::num::ParseIntError;
use axum::http::HeaderMap;
use reqwest::Client;
use serde_json::{json, Map};
use crate::entities::line::Line;

pub fn get_stan_api_calls_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8".parse().unwrap());
    headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
    
    headers
}

pub async fn request_presigned(client: &Client, url: String) -> anyhow::Result<String> {
    let json_response: Map<String, serde_json::Value> = client
        .post("https://nws-main.hove.io/api/presign")
        .json(&json!({
            "method": "POST",
            "path": url,
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

pub fn get_line_from_attribute(line_attribute: String, all_lines: &[Line]) -> Option<Line> {
    // If it starts with line:GST: -> line.id
    if line_attribute.starts_with("line:GST:") {
        return all_lines.iter().find(| l | {
            l.id == line_attribute
        }).cloned()
    };
    
    // If it parses into a number -> line.number
    let parsed: Result<usize, ParseIntError> = line_attribute.parse();
    if parsed.as_ref().is_ok() {
        return all_lines.iter().find(| l | {
            l.number == parsed.clone().unwrap()
        }).cloned()
    };
    
    // Else -> line.code
    all_lines.iter().find(| l | {
        l.code == line_attribute
    }).cloned()
}