use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Eq, ToSchema)]
pub struct Line {
    pub id: String,
    pub number: usize,
    pub name: String,
    pub code: String,
    pub color: String,
    pub text_color: String,
}

impl PartialEq<PartialLineInfo> for Line {
    fn eq(&self, other: &PartialLineInfo) -> bool {
        self.number == other.number || (self.color == other.color && self.text_color == other.text_color)
    }
}

impl PartialEq<ArrivalLineInfo> for Line {
    fn eq(&self, other: &ArrivalLineInfo) -> bool {
        match other {
            ArrivalLineInfo::Complete(li) => *self == *li,
            ArrivalLineInfo::Partial(pli) => *self == *pli
        }
    }
}

// Information about the line for arrivals
#[derive(Clone, ToSchema)]
pub enum ArrivalLineInfo {
    Complete(Line),
    Partial(PartialLineInfo)
}

impl Serialize for ArrivalLineInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ArrivalLineInfo", 1)?;

        match self {
            ArrivalLineInfo::Complete(l) => {
                state.serialize_field("line", &l)?;
            }
            ArrivalLineInfo::Partial(l) => {
                state.serialize_field("line", &l)?;
            }
        }

        state.end()
    }
}

#[derive(Serialize, Clone, ToSchema)]
pub struct PartialLineInfo {
    pub number: usize,
    pub color: String,
    pub text_color: String,
}