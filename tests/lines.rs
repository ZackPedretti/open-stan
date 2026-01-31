use axum::{
    body::{Body, to_bytes},
    http::Request,
};
use httpmock::{Method::GET, MockServer};
use open_stan::entities::{ApiState, Line};
use reqwest_rewire::RewireClient;
use serde_json::Result;
use std::{collections::HashMap, fs, sync::Arc};
use tower::ServiceExt;

mod common;

#[tokio::test]
async fn test_endpoint() {
    let mut rewire_map = HashMap::new();
    let mock_server = MockServer::start_async().await;
    let stan_mock = mock_server.mock(|when, then| {
        when.method(GET).path("/reseau-stan");
        then.status(200)
            .body(fs::read_to_string("./tests/data/homepage.html").unwrap());
    });
    rewire_map.insert(
        "https://www.reseau-stan.com/".to_string(),
        format!("{}", mock_server.url("/reseau-stan")),
    );

    let rewire_client = RewireClient::new(rewire_map);
    let client = Arc::new(reqwest_rewire::Client::TestClient(rewire_client));
    let state = ApiState { client };
    let router = common::init_router().with_state(state);

    let response = router
        .oneshot(Request::builder().uri("/lines").body(Body::empty()).unwrap())
        .await;
    assert!(response.is_ok());
    let response = response.unwrap();
    stan_mock.assert();

    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();

    let lines: Result<Vec<Line>> = serde_json::from_slice(&body_bytes);
    assert!(lines.is_ok());

    let lines = lines.unwrap();
    assert_eq!(lines.len(), 43);
}
