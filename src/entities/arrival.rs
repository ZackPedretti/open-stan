use crate::entities::line::ArrivalLineInfo;
use serde::Serialize;

#[derive(Serialize)]
pub struct Arrival {
    pub(crate) time: String,
    pub(crate) direction: String,
    pub(crate) static_time: bool,
    pub(crate) line_info: ArrivalLineInfo,
}
