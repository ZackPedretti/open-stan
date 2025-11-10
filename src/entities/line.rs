use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Eq)]
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
#[derive(Deserialize, Serialize, Clone)]
pub enum ArrivalLineInfo {
    Complete(Line),
    Partial(PartialLineInfo)
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PartialLineInfo {
    pub number: usize,
    pub color: String,
    pub text_color: String,
}