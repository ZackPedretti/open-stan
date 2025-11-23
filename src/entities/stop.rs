use serde::{Deserialize, Serialize};
use std::hash::Hash;
use utoipa::ToSchema;

#[derive(Eq, Hash, PartialEq, Serialize, Deserialize, Debug, ToSchema)]
pub struct Stop {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) label: String,
    pub(crate) coord: Coord,
}

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct Coord {
    lon: String,
    lat: String,
}

impl Stop {
    pub fn get_unique_identifier_from_str(id: &str) -> String {
        id.split(':').last().unwrap().to_string()
    }
}
