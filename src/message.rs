use crate::constants::MessageReferenceDate;
use crate::sections::{Header, Identification};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{Read, Seek};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub header: Header,
    pub identification: Identification,
}

impl Message {
    pub fn new<R: Read + Seek>(mut r: R) -> Result<Self, Box<dyn std::error::Error>> {
        let header = Header::from(&mut r);
        let identification = Identification::from(&mut r);
        Ok(Message {
            header,
            identification,
        })
    }
    pub fn json(&self) -> Result<String, serde_json::error::Error> {
        serde_json::to_string(&self)
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
