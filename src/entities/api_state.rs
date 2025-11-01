use reqwest::Client;

#[derive(Clone)]
pub struct ApiState {
    pub(crate) client: Client,
}