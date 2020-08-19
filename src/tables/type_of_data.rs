use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TypeOfData {
    AnalysisProducts,
    ForecastProducts,
    AnalysisAndForecastProducts,
    ControlForecastProducts,
    PerturbedForecastProducts,
    ControlAndPerturbedForecastProducts,
    ProcessedSatelliteObservations,
    ProcessedRadarObservations,
    EventProbability,
    Reserved,
    ReservedForLocalUse,
    ExperimentalProducts,
    Missing,
}

impl From<u8> for TypeOfData {
    fn from(n: u8) -> Self {
        match n {
            0 => TypeOfData::AnalysisProducts,
            1 => TypeOfData::ForecastProducts,
            2 => TypeOfData::AnalysisAndForecastProducts,
            3 => TypeOfData::ControlForecastProducts,
            4 => TypeOfData::PerturbedForecastProducts,
            5 => TypeOfData::ControlAndPerturbedForecastProducts,
            6 => TypeOfData::ProcessedSatelliteObservations,
            7 => TypeOfData::ProcessedRadarObservations,
            8 => TypeOfData::EventProbability,
            9..=191 => TypeOfData::Reserved,
            192 => TypeOfData::ExperimentalProducts,
            193..=254 => TypeOfData::ReservedForLocalUse,
            255 => TypeOfData::Missing,
        }
    }
}
