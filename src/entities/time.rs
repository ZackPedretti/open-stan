use serde::{Deserialize, Serialize};
use crate::entities::line::Line;

#[derive(Deserialize, Serialize)]
pub struct Time {
    pub(crate) time: String,
    pub(crate) direction: String,
    pub(crate) static_time: bool,
    pub(crate) line: Line,
}