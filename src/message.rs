use crate::constants::MessageReferenceDate;
use crate::sections::{
    DataRepresentation, GridDefinition, Header, Identification, LocalUse, ProductDefinition,
};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{Read, Seek};

type JsonResult = Result<String, serde_json::error::Error>;
pub trait ToJson {
    fn json(&self) -> JsonResult
    where
        Self: Sized + Serialize,
    {
        serde_json::to_string(&self)
    }
    fn pretty_json(&self) -> JsonResult
    where
        Self: Sized + Serialize,
    {
        serde_json::to_string_pretty(&self)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub header: Header,
    pub identification: Identification,
    pub local_use: LocalUse,
    pub grid_definition: GridDefinition,
    pub product_definition: ProductDefinition,
    pub data_representation: DataRepresentation,
}

impl ToJson for Message {}

impl Message {
    pub fn new<R: Read + Seek>(mut r: R) -> Self {
        let header = Header::from(&mut r);
        let identification = Identification::from(&mut r);
        let local_use = LocalUse::from(&mut r);
        let grid_definition = GridDefinition::from(&mut r);
        let product_definition = ProductDefinition::from(&mut r);
        let data_representation = DataRepresentation::from(&mut r);
        Message {
            header,
            identification,
            local_use,
            grid_definition,
            product_definition,
            data_representation,
        }
    }

    pub fn reference_date(&self) -> MessageReferenceDate {
        let Identification {
            year,
            month,
            day,
            hour,
            minute,
            second,
            significance_of_reference_time,
            ..
        } = &self.identification;
        let d = Utc.ymd(*year as i32, *month as u32, *day as u32).and_hms(
            *hour as u32,
            *minute as u32,
            *second as u32,
        );
        MessageReferenceDate(*significance_of_reference_time, d)
    }
}

#[cfg(test)]
mod test {
    use crate::constants::GribVersion;
    use super::Message;
    use std::io::Cursor;
    const SAMPLE_MESSAGE: &[u8] = include_bytes!("../sample-data/first-message.grib2");
    #[test]
    fn message_can_be_constructed() {
        let message = Message::new(Cursor::new(SAMPLE_MESSAGE));
        assert_eq!(message.header.version, GribVersion::Two)
    }
}
