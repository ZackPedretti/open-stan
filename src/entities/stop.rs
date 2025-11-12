use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[derive(Eq, Hash, PartialEq)]
pub struct Stop {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) label: String,
}

impl Stop {
   pub fn get_unique_identifier_from_str(id: &str) -> String {
       id.split(':').last().unwrap().to_string() 
    }
}