mod constants {
    #[derive(Debug, PartialEq)]
    pub enum GribVersion {
        One,
        Two,
        Invalid(u8),
    }

    impl From<u8> for GribVersion {
        fn from(n: u8) -> Self {
            match n {
                1 => GribVersion::One,
                2 => GribVersion::Two,
                n => GribVersion::Invalid(n),
            }
        }
    }
}

pub mod tables {
    #[derive(Debug, PartialEq)]
    pub enum ProductType {
        Meteorological,
        Hydrological,
        LandSurface,
        SatelliteRemoteSensing,
        SpaceWeather,
        Reserved,
        Oceanographic,
        ReservedForLocalUse,
        Missing,
    }
    impl From<u8> for ProductType {
        fn from(n: u8) -> Self {
            match n {
                0 => ProductType::Meteorological,
                1 => ProductType::Hydrological,
                2 => ProductType::LandSurface,
                3 => ProductType::SatelliteRemoteSensing,
                4 => ProductType::SpaceWeather,
                5..=9 | 11..=191 => ProductType::Reserved,
                10 => ProductType::Oceanographic,
                192..=254 => ProductType::ReservedForLocalUse,
                _ => ProductType::Missing,
            }
        }
    }
}

pub mod message {
    use super::constants::GribVersion;
    use super::tables::ProductType;
    use byteorder::{BigEndian, ReadBytesExt};
    use std::io::{Read, Seek};

    #[derive(Debug, PartialEq)]
    pub struct Header {
        pub magic: String,
        pub version: GribVersion,
        pub reserved: u16,
        pub product_type: ProductType,
        pub total_length: u64,
    }

    impl<R: Read + Seek> From<R> for Header {
        fn from(mut rdr: R) -> Self {
            let mut magic = [0u8; 4];
            rdr.read_exact(&mut magic)
                .expect("Couldn't read magic number of header");
            let reserved = rdr
                .read_u16::<BigEndian>()
                .expect("Couldn't read the reserved portion of the header");
            let product_type: ProductType =
                ProductType::from(rdr.read_u8().expect("Couldn't read the product type"));
            let version: GribVersion =
                GribVersion::from(rdr.read_u8().expect("Couldn't read the edition version"));
            let total_length = rdr
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

    #[derive(Debug, PartialEq)]
    pub struct Identification {
        length: u32,
        number_of_section: u8,
        center: u16,
        subcenter: u16,
        master_table_version: u8,
        local_table_version: u8,
        significance_of_reference_time: u8,
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        production_status_of_processed_data: u8,
        type_of_processed_data: u8,
        reserved: Option<Vec<u8>>,
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
                .expect("Cannot read number of center of Identification");
            let subcenter = rdr
                .read_u16::<BigEndian>()
                .expect("Cannot read number of subcenter of Identification");
            let master_table_version = rdr
                .read_u8()
                .expect("Cannot read number master table version of Identification");
            let local_table_version = rdr
                .read_u8()
                .expect("Cannot read local table version of Identification");
            let significance_of_reference_time = rdr
                .read_u8()
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
                .expect("Cannot read production status of processed data of Identification");
            let type_of_processed_data = rdr
                .read_u8()
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
                master_table_version,
                local_table_version,
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
}

use message::Message;

#[derive(Debug, PartialEq)]
struct Grib {
    message: Vec<Message>,
}

#[cfg(test)]
mod tests {
    use super::constants::GribVersion;
    use super::message::Message;
    use std::io::Cursor;
    const SAMPLE_MESSAGE: &[u8] = include_bytes!("../sample-data/first-message.grib2");
    #[test]
    fn message_can_be_constructed() {
        let message = Message::new(Cursor::new(SAMPLE_MESSAGE)).expect("cannot create a message");
        assert_eq!(message.header.version, GribVersion::Two)
    }
}
