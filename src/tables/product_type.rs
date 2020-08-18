#[derive(Debug, PartialEq)]
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
            0 => ProductType::Meteorological,
            1 => ProductType::Hydrological,
            2 => ProductType::LandSurface,
            3 => ProductType::SatelliteRemoteSensing,
            4 => ProductType::SpaceWeather,
            5..=9 | 11..=191 => ProductType::Reserved,
            10 => ProductType::Oceanographic,
            192..=254 => ProductType::ReservedForLocalUse,
            _ => ProductType::Missing,
        }
    }
}
