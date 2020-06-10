use serde_json::{Value};
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

fn read_json(local_path : &str, remote_path : &str) -> Result<(), Box<dyn Error>> {
    let local_file = File::open(local_path)?;
    let local_reader = BufReader::new(local_file);

    let remote_file = File::open(remote_path)?;
    let remote_reader = BufReader::new(remote_file);

    let v: Value = serde_json::from_reader(local_reader)?;
    if let Some(json) = v.as_object() {
        //do things
    }

    Ok(())
}

