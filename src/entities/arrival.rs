use crate::entities::line::ArrivalLineInfo;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[schema(description = "Represents a bus arrival at a specific stop, including the expected time, direction, and associated line information. The `static_time` field indicates whether the arrival time is real-time or scheduled.")]
pub struct Arrival {
    pub(crate) time: String,
    pub(crate) direction: String,
    pub(crate) static_time: bool,
    pub(crate) line: ArrivalLineInfo,
}
