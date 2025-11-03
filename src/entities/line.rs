use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Line {
    id: String,
    name: String,
    code: String,
    color: String,
    text_color: String,
}
