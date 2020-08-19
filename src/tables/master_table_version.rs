#[derive(Debug, PartialEq)]
pub enum GribMasterTableVersion {
    Experimental,
    VersionImplementedOn { day: u8, month: String, year: u16 },
    PreOperationalToBeImplementedByNextAmendment,
    FutureVersion,
    Missing,
}

impl From<u8> for GribMasterTableVersion {
    fn from(n: u8) -> Self {
        match n {
            0 => GribMasterTableVersion::Experimental,
            1 => GribMasterTableVersion::VersionImplementedOn {
                day: 7,
                month: "November".to_string(),
                year: 2001,
            },
            2 => GribMasterTableVersion::VersionImplementedOn {
                day: 4,
                month: "November".to_string(),
                year: 2003,
            },
            3 => GribMasterTableVersion::VersionImplementedOn {
                day: 2,
                month: "November".to_string(),
                year: 2005,
            },
            4 => GribMasterTableVersion::VersionImplementedOn {
                day: 7,
                month: "November".to_string(),
                year: 2007,
            },
            5 => GribMasterTableVersion::VersionImplementedOn {
                day: 4,
                month: "November".to_string(),
                year: 2009,
            },
            6 => GribMasterTableVersion::VersionImplementedOn {
                day: 15,
                month: "September".to_string(),
                year: 2010,
            },
            7 => GribMasterTableVersion::VersionImplementedOn {
                day: 4,
                month: "May".to_string(),
                year: 2011,
            },
            8 => GribMasterTableVersion::VersionImplementedOn {
                day: 8,
                month: "November".to_string(),
                year: 2011,
            },
            9 => GribMasterTableVersion::VersionImplementedOn {
                day: 2,
                month: "May".to_string(),
                year: 2012,
            },
            10 => GribMasterTableVersion::VersionImplementedOn {
                day: 7,
                month: "November".to_string(),
                year: 2012,
            },
            11 => GribMasterTableVersion::VersionImplementedOn {
                day: 8,
                month: "May".to_string(),
                year: 2013,
            },
            12 => GribMasterTableVersion::VersionImplementedOn {
                day: 14,
                month: "November".to_string(),
                year: 2013,
            },
            13 => GribMasterTableVersion::VersionImplementedOn {
                day: 7,
                month: "May".to_string(),
                year: 2014,
            },
            14 => GribMasterTableVersion::VersionImplementedOn {
                day: 5,
                month: "November".to_string(),
                year: 2014,
            },
            15 => GribMasterTableVersion::VersionImplementedOn {
                day: 6,
                month: "May".to_string(),
                year: 2015,
            },
            16 => GribMasterTableVersion::VersionImplementedOn {
                day: 11,
                month: "November".to_string(),
                year: 2015,
            },
            17 => GribMasterTableVersion::VersionImplementedOn {
                day: 4,
                month: "May".to_string(),
                year: 2016,
            },
            18 => GribMasterTableVersion::VersionImplementedOn {
                day: 2,
                month: "November".to_string(),
                year: 2016,
            },
            19 => GribMasterTableVersion::VersionImplementedOn {
                day: 3,
                month: "May".to_string(),
                year: 2017,
            },
            20 => GribMasterTableVersion::VersionImplementedOn {
                day: 8,
                month: "November".to_string(),
                year: 2017,
            },
            21 => GribMasterTableVersion::VersionImplementedOn {
                day: 2,
                month: "May".to_string(),
                year: 2018,
            },
            22 => GribMasterTableVersion::VersionImplementedOn {
                day: 7,
                month: "November".to_string(),
                year: 2018,
            },
            23 => GribMasterTableVersion::VersionImplementedOn {
                day: 15,
                month: "May".to_string(),
                year: 2019,
            },
            24 => GribMasterTableVersion::VersionImplementedOn {
                day: 06,
                month: "November".to_string(),
                year: 2019,
            },
            25 => GribMasterTableVersion::PreOperationalToBeImplementedByNextAmendment,
            26..=254 => GribMasterTableVersion::FutureVersion,
            255 => GribMasterTableVersion::Missing,
        }
    }
}
