use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum OriginatingSubcenter {
    Unknown,
    Undesignated,
    NCEPReanalysisProject,
    NCEPEnsembleProducts,
    NCEPCentralOperations,
    EnvironmentalModelingCenter,
    WeatherPredictionCenter,
    OceanPredictionCenter,
    ClimatePredictionCenter,
    AviationWeatherCenter,
    StormPredictionCenter,
    NationalHurricaneCenter,
    NWSTechniquesDevelopmentLaboratory,
    NESDISOfficeofResearchandApplications,
    FederalAviationAdministration,
    NWSMeteorologicalDevelopmentLaboratory,
    NorthAmericanRegionalReanalysisProject,
    SpaceWeatherPredictionCenter,
    ESRLGlobalSystemsDivision,
}

impl From<u16> for OriginatingSubcenter {
    fn from(n: u16) -> Self {
        match n {
            0 => Self::Undesignated,
            1 => Self::NCEPReanalysisProject,
            2 => Self::NCEPEnsembleProducts,
            3 => Self::NCEPCentralOperations,
            4 => Self::EnvironmentalModelingCenter,
            5 => Self::WeatherPredictionCenter,
            6 => Self::OceanPredictionCenter,
            7 => Self::ClimatePredictionCenter,
            8 => Self::AviationWeatherCenter,
            9 => Self::StormPredictionCenter,
            10 => Self::NationalHurricaneCenter,
            11 => Self::NWSTechniquesDevelopmentLaboratory,
            12 => Self::NESDISOfficeofResearchandApplications,
            13 => Self::FederalAviationAdministration,
            14 => Self::NWSMeteorologicalDevelopmentLaboratory,
            15 => Self::NorthAmericanRegionalReanalysisProject,
            16 => Self::SpaceWeatherPredictionCenter,
            17 => Self::ESRLGlobalSystemsDivision,
            _ => Self::Unknown,
        }
    }
}
