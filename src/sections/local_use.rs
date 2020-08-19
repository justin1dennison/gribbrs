use byteorder::{BigEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalUse {
    pub length: u32,
    pub number_of_section: u8,
    pub local_use: Vec<u8>,
}

impl<R: Read + Seek> From<R> for LocalUse {
    fn from(mut r: R) -> Self {
        let length = r
            .read_u32::<BigEndian>()
            .expect("Could not read the length of LocalUse");
        let number_of_section = r
            .read_u8()
            .expect("Could not read the number of section for LocalUse");

        let local_use = {
            let capacity = (length - 5) as usize;
            let mut xs = Vec::new();
            for _ in 0..capacity {
                let octet = r
                    .read_u8()
                    .expect("Could not read an octet of LocalUse local_use");
                xs.push(octet);
            }
            xs
        };
        LocalUse {
            length,
            number_of_section,
            local_use,
        }
    }
}
