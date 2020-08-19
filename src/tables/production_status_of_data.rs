use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProductionStatusOfData {
    OperationalProducts,
    OperationalTestProducts,
    ResearchProducts,
    ReanalysisProducts,
    TIGGE,
    TIGGETest,
    S2SOperationalProducts,
    S2STestProducts,
    UERRA,
    UERRATest,
    Reserved,
    ReservedforLocalUse,
    Missing,
}

impl From<u8> for ProductionStatusOfData {
    fn from(n: u8) -> Self {
        match n {
            0 => Self::OperationalProducts,
            1 => Self::OperationalTestProducts,
            2 => Self::ResearchProducts,
            3 => Self::ReanalysisProducts,
            4 => Self::TIGGE,
            5 => Self::TIGGETest,
            6 => Self::S2SOperationalProducts,
            7 => Self::S2STestProducts,
            8 => Self::UERRA,
            9 => Self::UERRATest,
            10..=191 => Self::Reserved,
            192..=254 => Self::ReservedforLocalUse,
            255 => Self::Missing,
        }
    }
}
