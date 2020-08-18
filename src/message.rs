use crate::sections::{ Header, Identification};
use std::io::{Read, Seek};

#[derive(Debug, PartialEq)]
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
}
