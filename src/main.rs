// To be used by git-difftool using extcmd flag

use std::env;
use std::error::Error;
use std::path::Path;
use std::ffi::OsStr;
use std::collections::HashSet;
use std::process::Command;
use ansi_term::Colour::Red;

mod formats;

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

fn call_sdiff(local_path : &str, remote_path : &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("sdiff").arg(&local_path).arg(&remote_path).output()?;
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let local_path = &args[1];
    let remote_path = &args[2];

    let base = env::var("BASE")?;
    let file_extension = extension_from_filename(&base);

    if !is_supported_format(file_extension) {
        // TODO should check for diff tools in computer and use what's available.
        println!("{}", Red.paint(format!("{} is not a supported file, we recommend using this tool only for the supported formats. Below is a normal sdiff.", base)));
        call_sdiff(&local_path, &remote_path)?;
    } else {
        formats::json::read_json(local_path, &remote_path)?;
    }

    Ok(())
}
