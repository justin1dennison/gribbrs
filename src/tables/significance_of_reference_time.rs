use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum SignificanceOfReferenceTime {
    Analysis,
    StartOfForecast,
    VerifyingTimeOfForecast,
    ObservationTime,
    Reserved,
    ReservedForLocalUse,
    Missing,
}

impl From<u8> for SignificanceOfReferenceTime {
    fn from(n: u8) -> Self {
        match n {
            0 => Self::Analysis,
            1 => Self::StartOfForecast,
            2 => Self::VerifyingTimeOfForecast,
            3 => Self::ObservationTime,
            4..=191 => Self::Reserved,
            192..=254 => Self::ReservedForLocalUse,
            255 => Self::Missing,
        }
    }
}
