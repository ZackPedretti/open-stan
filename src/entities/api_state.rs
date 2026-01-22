use std::sync::Arc;

#[derive(Clone)]
pub struct ApiState {
    pub client: Arc<reqwest_rewire::Client>,
}
