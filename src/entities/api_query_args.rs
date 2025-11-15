use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GetRemainingTimeToStopQueryArgs {
    pub(crate) stop: String,
    pub(crate) line: Option<String>,
    static_time: Option<bool>,
}

#[derive(Deserialize)]
pub struct GetStopOfLineQueryArgs {
    pub(crate) line: Option<String>,
}
