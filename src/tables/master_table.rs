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
            0 => Self::Experimental,
            1 => Self::VersionImplementedOn {
                day: 7,
                month: "November".to_string(),
                year: 2001,
            },
            2 => Self::VersionImplementedOn {
                day: 4,
                month: "November".to_string(),
                year: 2003,
            },
            3 => Self::VersionImplementedOn {
                day: 2,
                month: "November".to_string(),
                year: 2005,
            },
            4 => Self::VersionImplementedOn {
                day: 7,
                month: "November".to_string(),
                year: 2007,
            },
            5 => Self::VersionImplementedOn {
                day: 4,
                month: "November".to_string(),
                year: 2009,
            },
            6 => Self::VersionImplementedOn {
                day: 15,
                month: "September".to_string(),
                year: 2010,
            },
            7 => Self::VersionImplementedOn {
                day: 4,
                month: "May".to_string(),
                year: 2011,
            },
            8 => Self::VersionImplementedOn {
                day: 8,
                month: "November".to_string(),
                year: 2011,
            },
            9 => Self::VersionImplementedOn {
                day: 2,
                month: "May".to_string(),
                year: 2012,
            },
            10 => Self::VersionImplementedOn {
                day: 7,
                month: "November".to_string(),
                year: 2012,
            },
            11 => Self::VersionImplementedOn {
                day: 8,
                month: "May".to_string(),
                year: 2013,
            },
            12 => Self::VersionImplementedOn {
                day: 14,
                month: "November".to_string(),
                year: 2013,
            },
            13 => Self::VersionImplementedOn {
                day: 7,
                month: "May".to_string(),
                year: 2014,
            },
            14 => Self::VersionImplementedOn {
                day: 5,
                month: "November".to_string(),
                year: 2014,
            },
            15 => Self::VersionImplementedOn {
                day: 6,
                month: "May".to_string(),
                year: 2015,
            },
            16 => Self::VersionImplementedOn {
                day: 11,
                month: "November".to_string(),
                year: 2015,
            },
            17 => Self::VersionImplementedOn {
                day: 4,
                month: "May".to_string(),
                year: 2016,
            },
            18 => Self::VersionImplementedOn {
                day: 2,
                month: "November".to_string(),
                year: 2016,
            },
            19 => Self::VersionImplementedOn {
                day: 3,
                month: "May".to_string(),
                year: 2017,
            },
            20 => Self::VersionImplementedOn {
                day: 8,
                month: "November".to_string(),
                year: 2017,
            },
            21 => Self::VersionImplementedOn {
                day: 2,
                month: "May".to_string(),
                year: 2018,
            },
            22 => Self::VersionImplementedOn {
                day: 7,
                month: "November".to_string(),
                year: 2018,
            },
            23 => Self::VersionImplementedOn {
                day: 15,
                month: "May".to_string(),
                year: 2019,
            },
            24 => Self::VersionImplementedOn {
                day: 06,
                month: "November".to_string(),
                year: 2019,
            },
            25 => Self::PreOperationalToBeImplementedByNextAmendment,
            26..=254 => Self::FutureVersion,
            255 => Self::Missing,
        }
    }
}
