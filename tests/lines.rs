use std::collections::HashMap;

use reqwest_rewire::RewireClient;

mod common;

#[tokio::test]
async fn test_endpoint() {
    let mut rewire_map = HashMap::new();
    rewire_map.insert("https://www.reseau-stan.com/".to_string(), "".to_string());
    let rewire_client = RewireClient::new(rewire_map);
    let client = reqwest_rewire::Client::TestClient(rewire_client);
    let router = common::init_rewired_router(client);
}
