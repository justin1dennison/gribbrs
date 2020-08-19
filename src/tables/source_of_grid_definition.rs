use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SourceOfGridDefinition {
    SpecifiedInCodeTable31,
    PredeterminedGridDefinitionDefinedByOriginatingCenter,
    Reserved,
    ReservedForLocalUse,
    AGridDefinitionDoesNotApplyToThisProduct,
}

impl From<u8> for SourceOfGridDefinition {
    fn from(n: u8) -> Self {
        match n {
            0 => Self::SpecifiedInCodeTable31,
            1 => Self::PredeterminedGridDefinitionDefinedByOriginatingCenter,
            2..=191 => Self::Reserved,
            192..=254 => Self::ReservedForLocalUse,
            255 => Self::AGridDefinitionDoesNotApplyToThisProduct,
        }
    }
}
