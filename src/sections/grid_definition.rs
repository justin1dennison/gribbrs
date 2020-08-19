use crate::tables::{InterpretationOfNumbersAtEndOfSection3, SourceOfGridDefinition};
use byteorder::{BigEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GridDefinition {
    pub length: u32,
    pub number_of_section: u8,
    pub source: SourceOfGridDefinition,
    pub number_of_data_points: u32,
    pub number_of_optional_numbers: u8,
    pub interpretation_of_numbers_at_end_of_section_3: InterpretationOfNumbersAtEndOfSection3,
    pub template_no: u16,
}

impl<R: Read + Seek> From<R> for GridDefinition {
    fn from(mut r: R) -> Self {
        let length = r
            .read_u32::<BigEndian>()
            .expect("Could not read the length of GridDefinition");
        let number_of_section = r
            .read_u8()
            .expect("Could not read the number of section for GridDefinition");
        let source = r
            .read_u8()
            .map(SourceOfGridDefinition::from)
            .expect("Could not read the section for GridDefinition");
        let number_of_data_points = r
            .read_u32::<BigEndian>()
            .expect("Could not read the number of points for GridDefintion");
        let number_of_optional_numbers = r
            .read_u8()
            .expect("Could not read number of optional numbers for GridDefinition");
        let interpretation_of_numbers_at_end_of_section_3 = r
            .read_u8()
            .map(InterpretationOfNumbersAtEndOfSection3::from)
            .expect("Could not read interpretation for GridDefinition");
        let template_no = r
            .read_u16::<BigEndian>()
            .expect("Could not read template number for GridDefinition");
        GridDefinition {
            length,
            number_of_section,
            source,
            number_of_data_points,
            number_of_optional_numbers,
            interpretation_of_numbers_at_end_of_section_3,
            template_no,
        }
    }
}
