use gribbrs::message::Message;
use serde_json;
use std::io::Cursor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample = include_bytes!("../sample-data/first-message.grib2");
    let reader = Cursor::new(sample.to_vec());
    let message = Message::new(reader)?;
    println!("{:#?}", message);
    let json = serde_json::to_string_pretty(&message)?;
    println!("As json: \n{}", json);
    Ok(())
}
