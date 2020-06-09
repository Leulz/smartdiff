// To be used by git-difftool using extcmd flag

use serde_json::{Value};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::ffi::OsStr;
use std::collections::HashSet;
use std::process::Command;
use ansi_term::Colour::Red;

fn is_supported_format(format: &str, supported_formats: HashSet<String>) -> bool {
    supported_formats.contains(format)
}

fn extension_from_filename(filename: &str) -> &str {
    let extension = Path::new(filename)
        .extension()
        .and_then(OsStr::to_str);

    match extension {
        Some(e) => e,
        None => ""
    }   
}

fn read_json(local_path : &str, remote_path : &str) -> Result<(), Box<dyn Error>> {
    let local_file = File::open(local_path)?;
    let local_reader = BufReader::new(local_file);

    let remote_file = File::open(remote_path)?;
    let remote_reader = BufReader::new(remote_file);

    let v: Value = serde_json::from_reader(local_reader)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let mut supported_formats : HashSet<String> = HashSet::new();
    supported_formats.insert("json".to_string());
    
    let args: Vec<String> = env::args().collect();
    let local_path = &args[1];
    let remote_path = &args[2];
    let base = env::var("BASE")?;
    let file_extension = extension_from_filename(&base);

    if !is_supported_format(file_extension, supported_formats) {
        println!("{}", Red.paint(format!("{} is not a supported file, we recommend using this tool only for the supported formats. Below is a normal sdiff.", base)));
        // let command = format!("ls {} {}", local_path, remote_path);
        let output = Command::new("sdiff").arg(&local_path).arg(&remote_path).output()?;
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("{}", base);
    }
    
    // read_json(local_path, &remote_path)?;

    Ok(())
}
