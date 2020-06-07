use serde_json::{Value};
use text_io::read;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

fn read_json(file_path : &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let v: Value = serde_json::from_reader(reader)?;

    println!("a: {}, b: {}", v["a"], v["b"]);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path : String = read!();
    read_json(&file_path)?;

    Ok(())
}
