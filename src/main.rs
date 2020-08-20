use gribbrs::message::{Message, ToJson};
use std::io::Cursor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample = include_bytes!("../sample-data/first-message.grib2");
    let reader = Cursor::new(sample.to_vec());
    let message = Message::new(reader);
    println!("{:#?}", &message);
    let json = &message.pretty_json()?;
    println!("As json: \n{}", json);
    let reference_date = &message.reference_date();
    println!("Reference Date: {:#?}", reference_date);
    Ok(())
}
