mod json;
use crate::json::json::read_json;

use std::env;
use std::error::Error;
use std::path::Path;
use std::ffi::OsStr;
use std::collections::HashSet;

use smartdiff::formats;

#[cfg(test)] #[macro_use]
extern crate assert_matches;

fn is_supported_format(format: &str) -> bool {
    let supported_formats : HashSet<String> = formats::formats::get_supported_formats();
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

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let local_path = &args[1];
    let remote_path = &args[2];

    let base = env::var("BASE")?;
    let file_extension = extension_from_filename(&base);

    if is_supported_format(file_extension) {
        println!("{}", base);
        read_json(local_path, &remote_path)?;
    }

    Ok(())
}
