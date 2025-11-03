use serde::Serialize;

#[derive(Serialize)]
pub struct Stop {
    id: String,
    name: String,
    lon: i32,
    lat: i32,
}
