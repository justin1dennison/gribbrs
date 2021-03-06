use crate::constants::GribVersion;
use crate::tables::ProductType;
use byteorder::{BigEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Header {
    pub magic: String,
    pub version: GribVersion,
    pub reserved: u16,
    pub product_type: ProductType,
    pub total_length: u64,
}

impl<R: Read + Seek> From<R> for Header {
    fn from(mut r: R) -> Self {
        let mut magic = [0u8; 4];
        r.read_exact(&mut magic)
            .expect("Couldn't read magic number of header");
        let reserved = r
            .read_u16::<BigEndian>()
            .expect("Couldn't read the reserved portion of the header");
        let product_type: ProductType =
            ProductType::from(r.read_u8().expect("Couldn't read the product type"));
        let version: GribVersion =
            GribVersion::from(r.read_u8().expect("Couldn't read the edition version"));
        let total_length = r
            .read_u64::<BigEndian>()
            .expect("Could not determine the total length of the message");
        Header {
            magic: String::from_utf8_lossy(&magic).to_string(),
            version,
            reserved,
            product_type,
            total_length,
        }
    }
}
