use reqwest::Client;

#[derive(Clone)]
pub struct ApiState {
    pub client: Client,
}