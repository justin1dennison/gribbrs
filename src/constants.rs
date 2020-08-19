use crate::tables::SignificanceOfReferenceTime;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GribVersion {
    One,
    Two,
    Invalid(u8),
}

impl From<u8> for GribVersion {
    fn from(n: u8) -> Self {
        match n {
            1 => GribVersion::One,
            2 => GribVersion::Two,
            n => GribVersion::Invalid(n),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageReferenceDate(pub SignificanceOfReferenceTime, pub DateTime<Utc>);
