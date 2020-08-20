pub mod constants;
pub mod message;
pub mod sections;
pub mod tables;
use message::Message;

#[derive(Debug, PartialEq)]
struct Grib {
    messages: Vec<Message>,
}

use std::fs::File; 
use std::io::BufReader;

impl From<String> for Grib {
     fn from(path: String) -> Self {
        let mut messages: Vec<Message> = Vec::new();
        let file = File::open(path).expect("File not found");
        let reader = BufReader::new(file);
        let message = Message::new(reader);
        messages.push(message);
        return Grib { messages } 
     }
}

#[cfg(test)]
mod tests {
     use super::Grib; 

    #[test]
    fn create_a_grib_file_from_a_path() {
       let grib =  Grib::from(String::from("sample-data/first-message.grib2"));
       assert_eq!(grib.messages.len(), 1);        
    }
}
