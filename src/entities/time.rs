use serde::{Deserialize, Serialize};
use crate::entities::line::Line;

#[derive(Deserialize, Serialize)]
pub struct Time {
    line: Line,
    times: Vec<String>,
    direction: String
}