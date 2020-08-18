#[derive(Debug, PartialEq)]
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
