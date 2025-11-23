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
    /// Returns the unique identifier of a stop from a string
    /// 
    /// For a stop having this ID:
    /// `stop_point:GST:SP:HLGRU1`
    /// 
    /// Its unique identifier is:
    /// `HLGRU1`
    /// 
    /// # Panics
    /// Panics if no identifier was found
    #[must_use] 
    pub fn get_unique_identifier_from_str(id: &str) -> String {
        id.split(':').next_back().unwrap().to_string()
    }
}
