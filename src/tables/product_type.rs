use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProductType {
    Meteorological,
    Hydrological,
    LandSurface,
    SatelliteRemoteSensing,
    SpaceWeather,
    Reserved,
    Oceanographic,
    ReservedForLocalUse,
    Missing,
}

impl From<u8> for ProductType {
    fn from(n: u8) -> Self {
        match n {
            0 => Self::Meteorological,
            1 => Self::Hydrological,
            2 => Self::LandSurface,
            3 => Self::SatelliteRemoteSensing,
            4 => Self::SpaceWeather,
            5..=9 | 11..=191 => Self::Reserved,
            10 => Self::Oceanographic,
            192..=254 => Self::ReservedForLocalUse,
            _ => Self::Missing,
        }
    }
}
