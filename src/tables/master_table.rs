use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GribMasterTable {
    Experimental,
    VersionImplementedOn { day: u8, month: String, year: u16 },
    PreOperationalToBeImplementedByNextAmendment,
    FutureVersion,
    Missing,
}

impl From<u8> for GribMasterTable {
    fn from(n: u8) -> Self {
        match n {
            0 => GribMasterTable::Experimental,
            1 => GribMasterTable::VersionImplementedOn {
                day: 7,
                month: "November".to_string(),
                year: 2001,
            },
            2 => GribMasterTable::VersionImplementedOn {
                day: 4,
                month: "November".to_string(),
                year: 2003,
            },
            3 => GribMasterTable::VersionImplementedOn {
                day: 2,
                month: "November".to_string(),
                year: 2005,
            },
            4 => GribMasterTable::VersionImplementedOn {
                day: 7,
                month: "November".to_string(),
                year: 2007,
            },
            5 => GribMasterTable::VersionImplementedOn {
                day: 4,
                month: "November".to_string(),
                year: 2009,
            },
            6 => GribMasterTable::VersionImplementedOn {
                day: 15,
                month: "September".to_string(),
                year: 2010,
            },
            7 => GribMasterTable::VersionImplementedOn {
                day: 4,
                month: "May".to_string(),
                year: 2011,
            },
            8 => GribMasterTable::VersionImplementedOn {
                day: 8,
                month: "November".to_string(),
                year: 2011,
            },
            9 => GribMasterTable::VersionImplementedOn {
                day: 2,
                month: "May".to_string(),
                year: 2012,
            },
            10 => GribMasterTable::VersionImplementedOn {
                day: 7,
                month: "November".to_string(),
                year: 2012,
            },
            11 => GribMasterTable::VersionImplementedOn {
                day: 8,
                month: "May".to_string(),
                year: 2013,
            },
            12 => GribMasterTable::VersionImplementedOn {
                day: 14,
                month: "November".to_string(),
                year: 2013,
            },
            13 => GribMasterTable::VersionImplementedOn {
                day: 7,
                month: "May".to_string(),
                year: 2014,
            },
            14 => GribMasterTable::VersionImplementedOn {
                day: 5,
                month: "November".to_string(),
                year: 2014,
            },
            15 => GribMasterTable::VersionImplementedOn {
                day: 6,
                month: "May".to_string(),
                year: 2015,
            },
            16 => GribMasterTable::VersionImplementedOn {
                day: 11,
                month: "November".to_string(),
                year: 2015,
            },
            17 => GribMasterTable::VersionImplementedOn {
                day: 4,
                month: "May".to_string(),
                year: 2016,
            },
            18 => GribMasterTable::VersionImplementedOn {
                day: 2,
                month: "November".to_string(),
                year: 2016,
            },
            19 => GribMasterTable::VersionImplementedOn {
                day: 3,
                month: "May".to_string(),
                year: 2017,
            },
            20 => GribMasterTable::VersionImplementedOn {
                day: 8,
                month: "November".to_string(),
                year: 2017,
            },
            21 => GribMasterTable::VersionImplementedOn {
                day: 2,
                month: "May".to_string(),
                year: 2018,
            },
            22 => GribMasterTable::VersionImplementedOn {
                day: 7,
                month: "November".to_string(),
                year: 2018,
            },
            23 => GribMasterTable::VersionImplementedOn {
                day: 15,
                month: "May".to_string(),
                year: 2019,
            },
            24 => GribMasterTable::VersionImplementedOn {
                day: 06,
                month: "November".to_string(),
                year: 2019,
            },
            25 => GribMasterTable::PreOperationalToBeImplementedByNextAmendment,
            26..=254 => GribMasterTable::FutureVersion,
            255 => GribMasterTable::Missing,
        }
    }
}
