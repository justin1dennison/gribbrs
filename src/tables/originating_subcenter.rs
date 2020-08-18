#[derive(Debug, PartialEq)]
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
            0 => OriginatingSubcenter::Undesignated,
            1 => OriginatingSubcenter::NCEPReanalysisProject,
            2 => OriginatingSubcenter::NCEPEnsembleProducts,
            3 => OriginatingSubcenter::NCEPCentralOperations,
            4 => OriginatingSubcenter::EnvironmentalModelingCenter,
            5 => OriginatingSubcenter::WeatherPredictionCenter,
            6 => OriginatingSubcenter::OceanPredictionCenter,
            7 => OriginatingSubcenter::ClimatePredictionCenter,
            8 => OriginatingSubcenter::AviationWeatherCenter,
            9 => OriginatingSubcenter::StormPredictionCenter,
            10 => OriginatingSubcenter::NationalHurricaneCenter,
            11 => OriginatingSubcenter::NWSTechniquesDevelopmentLaboratory,
            12 => OriginatingSubcenter::NESDISOfficeofResearchandApplications,
            13 => OriginatingSubcenter::FederalAviationAdministration,
            14 => OriginatingSubcenter::NWSMeteorologicalDevelopmentLaboratory,
            15 => OriginatingSubcenter::NorthAmericanRegionalReanalysisProject,
            16 => OriginatingSubcenter::SpaceWeatherPredictionCenter,
            17 => OriginatingSubcenter::ESRLGlobalSystemsDivision,
            _ => OriginatingSubcenter::Unknown,
        }
    }
}
