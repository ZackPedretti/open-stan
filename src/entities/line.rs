use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Line {
    pub id: String,
    pub name: String,
    pub code: String,
    pub color: String,
    pub text_color: String,
}
