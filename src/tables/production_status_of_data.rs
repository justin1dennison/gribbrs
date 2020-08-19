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
            0 => ProductionStatusOfData::OperationalProducts,
            1 => ProductionStatusOfData::OperationalTestProducts,
            2 => ProductionStatusOfData::ResearchProducts,
            3 => ProductionStatusOfData::ReanalysisProducts,
            4 => ProductionStatusOfData::TIGGE,
            5 => ProductionStatusOfData::TIGGETest,
            6 => ProductionStatusOfData::S2SOperationalProducts,
            7 => ProductionStatusOfData::S2STestProducts,
            8 => ProductionStatusOfData::UERRA,
            9 => ProductionStatusOfData::UERRATest,
            10..=191 => ProductionStatusOfData::Reserved,
            192..=254 => ProductionStatusOfData::ReservedforLocalUse,
            255 => ProductionStatusOfData::Missing,
        }
    }
}
