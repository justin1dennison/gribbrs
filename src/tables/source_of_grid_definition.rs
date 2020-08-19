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
            0 => SourceOfGridDefinition::SpecifiedInCodeTable31,
            1 => SourceOfGridDefinition::PredeterminedGridDefinitionDefinedByOriginatingCenter,
            2..=191 => SourceOfGridDefinition::Reserved,
            192..=254 => SourceOfGridDefinition::ReservedForLocalUse,
            255 => SourceOfGridDefinition::AGridDefinitionDoesNotApplyToThisProduct,
        }
    }
}
