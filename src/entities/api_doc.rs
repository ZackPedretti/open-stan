use crate::endpoints::arrivals;
use crate::endpoints::{lines, stops};
use crate::entities::stop::Stop;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        stops::get_stops,
        lines::get_lines,
        arrivals::get_arrivals,
    ),
    components(
        schemas(Stop)
    ),
    tags(
        (name = "stops", description = "Stop-related endpoints"),
        (name = "lines", description = "Line-related endpoints"),
        (name = "arrivals", description = "Arrivals and predictions")
    )
)]
pub struct ApiDoc;
