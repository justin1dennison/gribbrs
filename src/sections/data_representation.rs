use byteorder::{BigEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DataRepresentation {
    pub length: u32,
    pub number_of_section: u8,
    pub number_of_data_points: u32,
    pub template_no: u16,
    //pub reserved: Vec<u8>
}

impl<R: Read + Seek> From<R> for DataRepresentation {
    fn from(mut r: R) -> Self {
        let length = r
            .read_u32::<BigEndian>()
            .expect("Couldn't read length of DataRepresentation");
        let number_of_section = r
            .read_u8()
            .expect("Couldn't read number of section of DataRepresentation");
        let number_of_data_points = r
            .read_u32::<BigEndian>()
            .expect("Couldn't read number of data points of DataRepresentation");
        let template_no = r
            .read_u16::<BigEndian>()
            .expect("Couldn't read template number of DataRepresentation");
        //let reserved = {
        //  let mut xs = Vec::new();
        //  let capacity = (length - 11) as usize;
        //  for _ in 0..capacity {
        //    let byte = r.read_u8().unwrap();
        //    xs.push(byte);
        //  }
        //  xs
        //};
        DataRepresentation {
            length,
            number_of_section,
            number_of_data_points,
            template_no,
            //reserved
        }
    }
}
