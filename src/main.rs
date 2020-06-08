// To be used by git-difftool using extcmd flag

use serde_json::{Value};
use std::env;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

// TODO Verify type of file in order to get diff

fn read_json(local_path : &str, remote_path : &str) -> Result<(), Box<dyn Error>> {
    let local_file = File::open(local_path)?;
    let local_reader = BufReader::new(local_file);

    let remote_file = File::open(remote_path)?;
    let remote_reader = BufReader::new(remote_file);

    // let v: Value = serde_json::from_reader(reader)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let local_path = &args[1];
    let remote_path = &args[2];
    let base = env::var("BASE")?;
    println!("{}", base);
    
    read_json(local_path, remote_path)?;

    Ok(())
}
