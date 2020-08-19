use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InterpretationOfNumbersAtEndOfSection3 {
    NoAppendedList,
    FullCoordinateCircles,
    ExtremeCoordinateValues,
    ActualLatitudesInMicroDegrees,
    Reserved,
    Missing,
}

impl From<u8> for InterpretationOfNumbersAtEndOfSection3 {
    fn from(n: u8) -> Self {
        match n {
            0 => Self::NoAppendedList,
            1 => Self::FullCoordinateCircles,
            2 => Self::ExtremeCoordinateValues,
            3 => Self::ActualLatitudesInMicroDegrees,
            4..=254 => Self::Reserved,
            _ => Self::Missing,
        }
    }
}
