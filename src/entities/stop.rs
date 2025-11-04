use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Stop {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) label: String,
}
