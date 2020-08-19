use crate::tables::{
    GribLocalTable, GribMasterTable, OriginatingCenter, OriginatingSubcenter,
    ProductionStatusOfData, SignificanceOfReferenceTime, TypeOfData,
};
use byteorder::{BigEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Identification {
    pub length: u32,
    pub number_of_section: u8,
    pub center: OriginatingCenter,
    pub subcenter: OriginatingSubcenter,
    pub master_table: GribMasterTable,
    pub local_table: GribLocalTable,
    pub significance_of_reference_time: SignificanceOfReferenceTime,
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub production_status_of_processed_data: ProductionStatusOfData,
    pub type_of_processed_data: TypeOfData,
    pub reserved: Option<Vec<u8>>,
}

impl<R: Read + Seek> From<R> for Identification {
    fn from(mut rdr: R) -> Self {
        let length = rdr
            .read_u32::<BigEndian>()
            .expect("Cannot read length of Identification section");
        let number_of_section = rdr
            .read_u8()
            .expect("Cannot read number of section of Identification");
        let center = rdr
            .read_u16::<BigEndian>()
            .map(OriginatingCenter::from)
            .expect("Cannot read number of center of Identification");
        let subcenter = rdr
            .read_u16::<BigEndian>()
            .map(OriginatingSubcenter::from)
            .expect("Cannot read number of subcenter of Identification");
        let master_table = rdr
            .read_u8()
            .map(GribMasterTable::from)
            .expect("Cannot read number master table version of Identification");
        let local_table = rdr
            .read_u8()
            .map(GribLocalTable::from)
            .expect("Cannot read local table version of Identification");
        let significance_of_reference_time = rdr
            .read_u8()
            .map(SignificanceOfReferenceTime::from)
            .expect("Cannot read significance of reference time of Identification");
        let year = rdr
            .read_u16::<BigEndian>()
            .expect("Cannot read year of Identification");
        let month = rdr.read_u8().expect("Cannot read month of Identification");
        let day = rdr.read_u8().expect("Cannot read day of Identification");
        let hour = rdr.read_u8().expect("Cannot read hour of Identification");
        let minute = rdr.read_u8().expect("Cannot read minute of Identification");
        let second = rdr.read_u8().expect("Cannot read second of Identification");
        let production_status_of_processed_data = rdr
            .read_u8()
            .map(ProductionStatusOfData::from)
            .expect("Cannot read production status of processed data of Identification");
        let type_of_processed_data = rdr
            .read_u8()
            .map(TypeOfData::from)
            .expect("Cannot read type of processed data of Identification");
        let reserved = if length == 21 {
            None
        } else {
            let capacity = (length - 21) as usize;
            let mut xs = Vec::with_capacity(capacity);
            rdr.read(&mut xs)
                .expect("Cannot read reserved section of Identification");
            Some(xs)
        };
        Identification {
            length,
            number_of_section,
            center,
            subcenter,
            master_table,
            local_table,
            significance_of_reference_time,
            year,
            month,
            day,
            hour,
            minute,
            second,
            production_status_of_processed_data,
            type_of_processed_data,
            reserved,
        }
    }
}
