use serde::{Deserialize, Serialize};
use serde;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Eq, ToSchema)]
#[schema(description = "Represents a bus line with its name, color, and identifying metadata.")]
pub struct Line {
    #[schema(example = "line:GST:1-97")]
    pub id: String,
    #[schema(example = "2484")]
    pub number: usize,
    #[schema(example = "Vandoeuvre Brabois Hôpitaux - Essey Mouzimpré")]
    pub name: String,
    #[schema(example = "T1")]
    pub code: String,
    #[schema(example = "#E30613")]
    pub color: String,
    #[schema(example = "#FFFFFF")]
    pub text_color: String,
}

impl PartialEq<PartialLineInfo> for Line {
    fn eq(
        &self,
        other: &PartialLineInfo,
    ) -> bool {
        self.number == other.number || (self.color == other.color && self.text_color == other.text_color)
    }
}

impl PartialEq<ArrivalLineInfo> for Line {
    fn eq(
        &self,
        other: &ArrivalLineInfo,
    ) -> bool {
        match other {
            ArrivalLineInfo::Complete(li) => *self == *li,
            ArrivalLineInfo::Partial(pli) => *self == *pli,
        }
    }
}

#[derive(Clone, ToSchema, Serialize)]
#[serde(untagged)]
#[schema(description = "Line information returned by arrival endpoint only.")]
pub enum ArrivalLineInfo {
    Complete(Line),
    Partial(PartialLineInfo),
}

#[derive(Serialize, Clone, ToSchema)]
#[schema(description = "Partial line information, missing line ID, name, and code. This happens when the STAN API returns arrivals from bus lines that are not returned by the bus lines request.")]
pub struct PartialLineInfo {
    #[schema(example = "2484")]
    pub number: usize,
    #[schema(example = "#E30613")]
    pub color: String,
    #[schema(example = "#FFFFFF")]
    pub text_color: String,
}
