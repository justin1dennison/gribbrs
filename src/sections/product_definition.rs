use byteorder::{BigEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductDefinition {
    pub length: u32,
    pub number_of_section: u8,
    pub number_of_coordinate_values_after_template: u16,
    pub product_definition_template_number: u16,
    pub reserved: Vec<u8>,
}

impl<R: Read + Seek> From<R> for ProductDefinition {
    fn from(mut r: R) -> Self {
        let length = r
            .read_u32::<BigEndian>()
            .expect("Couldn't read length of ProductDefinition");
        let number_of_section = r
            .read_u8()
            .expect("Couldn't read number of section of ProductDefinition");
        let number_of_coordinate_values_after_template = r.read_u16::<BigEndian>().expect(
            "Couldn't read number of coordinate values after template of ProductDefinition",
        );
        let product_definition_template_number = r
            .read_u16::<BigEndian>()
            .expect("Couldn't read product definition template number of Product Definition");
        let reserved = {
            let mut xs = Vec::new();
            let capacity = (length - 9) as usize;
            for _ in 0..capacity {
                let byte = r.read_u8().unwrap();
                xs.push(byte);
            }
            xs
        };
        ProductDefinition {
            length,
            number_of_section,
            number_of_coordinate_values_after_template,
            product_definition_template_number,
            reserved,
        }
    }
}
