use axum::body::to_bytes;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, Router};
use httptest::matchers::request;
use httptest::responders::status_code;
use httptest::{Expectation, Server};
use open_stan::endpoints::lines::request_lines;
use open_stan::entities::api_state::ApiState;
use open_stan::entities::line::Line;
use tower::ServiceExt;


async fn get_lines(
    State(state): State<ApiState>,
    url: String,
) -> impl IntoResponse {
    match request_lines(&state.client, &url).await {
        Ok(v) => Json(v).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[tokio::test]
async fn integration_test() {
    let server = Server::run();
    let html = std::fs::read_to_string("tests/data/homepage.html").unwrap();
    server.expect(Expectation::matching(request::method_path("GET", "/")).respond_with(status_code(200).body(html)));
    let client = reqwest::Client::builder().build().unwrap();
    let api_state = ApiState { client };
    let url = server.url("/").to_string();
    let app = Router::new()
        .route(
            "/",
            axum::routing::get({
                let url = url.clone();
                move |state: State<ApiState>| get_lines(state, url.clone())
            }),
        )
        .with_state(api_state);

    let response = app
        .oneshot(axum::http::Request::get("/").body(axum::body::Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let lines: Vec<Line> = serde_json::from_slice(&body).unwrap();
    assert_ne!(lines.len(), 0);
    assert_eq!(lines.len(), 43);
}
