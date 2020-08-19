use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GribLocalTable {
    Unused,
    Version(u8),
    Missing,
}

impl From<u8> for GribLocalTable {
    fn from(n: u8) -> Self {
        match n {
            0 => Self::Unused,
            1..=254 => Self::Version(n),
            _ => Self::Missing,
        }
    }
}
