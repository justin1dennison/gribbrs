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
            0 => SignificanceOfReferenceTime::Analysis,
            1 => SignificanceOfReferenceTime::StartOfForecast,
            2 => SignificanceOfReferenceTime::VerifyingTimeOfForecast,
            3 => SignificanceOfReferenceTime::ObservationTime,
            4..=191 => SignificanceOfReferenceTime::Reserved,
            192..=254 => SignificanceOfReferenceTime::ReservedForLocalUse,
            255 => SignificanceOfReferenceTime::Missing,
        }
    }
}
