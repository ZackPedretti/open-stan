use anyhow::anyhow;
use crate::entities::api_state::ApiState;
use crate::entities::stop::Stop;
use crate::entities::arrival::Arrival;
use crate::utils::{get_line_from_attribute, get_stan_api_calls_headers, get_style_value_from_elt};
use axum::{Json, Router};
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::routing::get;
use reqwest::{Client, StatusCode};
use scraper::{ElementRef, Html, Selector};
use crate::endpoints::lines::request_lines;
use crate::entities::api_query_args::GetRemainingTimeToStopQueryArgs;
use crate::entities::line::{ArrivalLineInfo, PartialLineInfo};

pub fn router() -> Router<ApiState> {
    let router: Router<ApiState> = Router::new().route("/", get(get_remaining_times_to_stop));

    router
}


async fn get_remaining_times_to_stop(
    State(state): State<ApiState>,
    Query(query): Query<GetRemainingTimeToStopQueryArgs>,
) -> impl IntoResponse {
    match request_remaining_times_to_stop(query.stop, query.line, &state.client)
        .await {
        Ok(v) => Json(v).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn request_remaining_times_to_stop(
    stop: String,
    line: Option<String>,
    client: &Client,
) -> anyhow::Result<Vec<Arrival>> {
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

    let all_lines = request_lines(client).await?;
    
    let specified_line = match line {
        None => None,
        Some(l) => {get_line_from_attribute(l, &all_lines)}
    };
    
    let document = Html::parse_document(&html_text);

    let time_selector =
        Selector::parse("ul li").expect("There was something wrong with the HTML document.");

    let mut times: Vec<Arrival> = vec![];
    
    for elt in document.select(&time_selector) {
        let line_num = get_line_num_from_elt(&elt).unwrap();
        let line = match get_line_from_attribute(line_num.clone(), &all_lines) {
            None => ArrivalLineInfo::Partial(get_partial_line_info_from_elt(&elt)),
            Some(l) => ArrivalLineInfo::Complete(l)
        };
        if let Some(ref spec_line) = specified_line {
            if *spec_line != line {
                continue;
            }
        }
        let direction = get_direction_from_elt(&elt).unwrap_or("No destination was specified.".into());
        let time_values = match get_arrival_times_from_elt(&elt) {
            Some(t) => t,
            None => { return Err(anyhow!("No time was provided by the HTML document. Make sure you entered an existing stop id.")); }
        };
        let is_static_time = get_if_static_time(&elt);
        
        for t in time_values {
            times.push(Arrival {
                time: t,
                direction: direction.clone(),
                static_time: is_static_time,
                line_info: line.clone(),
            })
        }
    }

    Ok(times)
}

fn get_direction_from_elt(elt: &ElementRef) -> Option<String> {
    let direction_sub_selector = Selector::parse(".tpsreel-destination span").expect("There was something wrong with the HTML document.");
    let mut elt = elt.select(&direction_sub_selector);

    match elt.next() {
        None => { None }
        Some(v) => { Some(v.text().next().unwrap().to_string()) }
    }
}

fn get_arrival_times_from_elt(elt: &ElementRef) -> Option<Vec<String>> {
    let time_sub_selector = Selector::parse(".tpsreel-temps .tpsreel-temps-item").unwrap();
    
    let elts = elt.select(&time_sub_selector);
    
    let mut times: Vec<String> = vec![];
    
    for elt in elts {
        let time = match elt.text().next() {
            None => "< 1 min".to_string(),
            Some(t) => t.to_string(),
        };
        times.push(time);
    }
    Some(times)
}

fn get_if_static_time(elt: &ElementRef) -> bool {
    let static_time_selector = Selector::parse(".tpsreel-temps-item-tpstheorique").unwrap();
    let mut elt = elt.select(&static_time_selector);
    elt.next().is_some()
}

fn get_line_num_from_elt(elt: &ElementRef) -> Option<String> {
    let line_number_selector = Selector::parse(".tpsreel-ligne").unwrap();
    let mut elt = elt.select(&line_number_selector);
    elt.next().map(|elt| elt.attr("id").unwrap().to_string()[9..].trim().to_string())
}

fn get_partial_line_info_from_elt(elt: &ElementRef) -> PartialLineInfo {
    let line_info_selector = Selector::parse(".tpsreel-ligne").unwrap();
    let elt = elt.select(&line_info_selector).next().unwrap();

    let style = elt.value().attr("style").unwrap_or("");
    PartialLineInfo {
        number: elt.value().attr("id").unwrap_or("").to_string()[9..].parse().unwrap_or(0),
        color: get_style_value_from_elt(style, "background-color").unwrap_or("".to_string()),
        text_color: get_style_value_from_elt(style, "color").unwrap_or("".to_string()),
    }
}