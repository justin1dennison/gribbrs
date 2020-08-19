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
            0 => GribLocalTable::Unused,
            1..=254 => GribLocalTable::Version(n),
            _ => GribLocalTable::Missing,
        }
    }
}
