use gribbrs::message::Message;
use std::io::Cursor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample = include_bytes!("../sample-data/first-message.grib2");
    let reader = Cursor::new(sample.to_vec());
    let message = Message::new(reader)?;
    println!("{:#?}", message);
    Ok(())
}
