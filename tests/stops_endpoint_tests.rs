// CANNOT BE EASILY TESTED AT THE MOMENT
// I WILL CREATE A CRATE TO MOCK THIS MORE EASILY
// use axum::extract::State;
// use axum::Router;
// use httptest::matchers::json_decoded;
// use httptest::matchers::{matches, request};
// use httptest::responders::status_code;
// use httptest::{Expectation, Server, all_of};
// use reqwest::Client;
// use serde::Deserialize;
// use serde_json::json;
// use open_stan::entities::api_state::ApiState;
// use open_stan::entities::stop::Stop;
// 
// #[derive(Deserialize)]
// struct RequestBody {
//     #[serde(rename = "clientName")]
//     client_name: String,
//     href: String,
//     #[serde(rename = "presignedUrl")]
//     presigned_url: String,
// }
// 
// #[tokio::test]
// async fn integration_test() {
//     let server = Server::run();
//     let presign_response = json!({
//         "url": "/v1/coverage/fr-ne-nancy/lines/line:GST:1-97/stop_areas?count=100&depth=3&x_client=stan&x_key_id=d46f3a6819938fded38b669febb55aca4da5202ae3a158acd8ba4a388f6eaed6&x_ts=1764099280&x_ttl=120&x_m=POST&x_bsha256=e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855&x_sig=FkwbUmPqPkzmjUNQfl3Ak1WQXpVwW3A76SoVjyq80dE%3D"
//     });
//     server.expect(
//         Expectation::matching(all_of![request::method_path("POST", "/presign")])
//             .respond_with(status_code(200).body(presign_response.to_string())),
//     );
//     server.expect(
//         Expectation::matching(all_of![
//             request::method_path("POST", "/proxy"),
//             // request::body(|body: &RequestBody| body.client_name == "stan")
//             // BODY SHOULD BE:
//             // {
//             //     "presignedUrl": "/v1/coverage/fr-ne-nancy/lines/line:GST:1-97/stop_areas?count=100&depth=3&x_client=stan&x_key_id=d46f3a6819938fded38b669febb55aca4da5202ae3a158acd8ba4a388f6eaed6&x_ts=1764099280&x_ttl=120&x_m=POST&x_bsha256=e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855&x_sig=FkwbUmPqPkzmjUNQfl3Ak1WQXpVwW3A76SoVjyq80dE%3D",
//             //     "href": "https://api.navitia.io/v1/coverage/fr-ne-nancy/lines/line:GST:1-97/stop_areas?count=100&depth=3",
//             //     "clientName": "stan"
//             // }
//         ])
//         .respond_with(status_code(200).body(std::fs::read_to_string("tests/data/stops.json").unwrap())),
//     );
//     let client = reqwest::Client::builder().build().unwrap();
//     let api_state = ApiState { client };
//     let url = server.url("/").to_string();
// 
//     let app = Router::new()
//         .route(
//             "/",
//             axum::routing::get({
//                 let url = url.clone();
//                 move |state: State<ApiState>| get_stops(state, url.clone())
//             }),
//         )
//         .with_state(api_state);
// }