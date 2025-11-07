use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Eq)]
pub struct Line {
    pub id: String,
    pub number: usize,
    pub name: String,
    pub code: String,
    pub color: String,
    pub text_color: String,
}
