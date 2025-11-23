use crate::entities::line::ArrivalLineInfo;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Arrival {
    pub(crate) time: String,
    pub(crate) direction: String,
    pub(crate) static_time: bool,
    #[serde(flatten)]
    pub(crate) line_info: ArrivalLineInfo,
}
