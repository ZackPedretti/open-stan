use serde::{Deserialize, Serialize};
use std::hash::Hash;
use utoipa::ToSchema;

#[derive(Eq, Hash, PartialEq, Serialize, Deserialize, Debug, ToSchema)]
#[schema(description = "Represents a bus stop with its name, label, coordinates, and identifier.")]
pub struct Stop {
    #[schema(example = "stop_area:GST:SA:HDRUZ")]
    pub(crate) id: String,
    #[schema(example = "Rouzé")]
    pub(crate) name: String,
    #[schema(example = "Rouzé (Houdemont)")]
    pub(crate) label: String,
    pub(crate) coord: Coord,
}

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
#[schema(description = "Coordinates of a bus stop object. WARNING: the attributes are strings, since it is what the Navitia API returns.")]
pub struct Coord {
    #[schema(example = "6.177644")]
    lon: String,
    #[schema(example = "48.638481")]
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
