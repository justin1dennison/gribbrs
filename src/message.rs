use crate::sections::{Header, Identification};
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek};
use serde_json;

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
    pub fn json(self) -> Result<String, serde_json::error::Error> {
        serde_json::to_string(&self)
    }
}
