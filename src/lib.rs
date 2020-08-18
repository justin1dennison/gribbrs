pub mod constants;
pub mod message;
pub mod sections;
pub mod tables;
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
