use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use ansi_term::Colour::{Green, Red};
use serde_json::map::Map;
use serde_json::Value;

use crate::formats::json::*;

const LOCAL: &str = "local";
const REMOTE: &str = "remote";

macro_rules! local_remote_choice {
    ($origin:expr, $local:expr, $remote:expr) => {
        match $origin {
            LOCAL => Ok($local),
            REMOTE => Ok($remote),
            _ => Err("Invalid origin! Must be either LOCAL or REMOTE."),
        }
    };
}

fn get_diff_symbol(origin: &str) -> Result<&str, Box<dyn Error>> {
    let symbol = local_remote_choice!(origin, "-", "+")?;

    Ok(symbol)
}

fn get_diff_color(origin: &str) -> Result<ansi_term::Colour, Box<dyn Error>> {
    let color = local_remote_choice!(origin, Red, Green)?;

    Ok(color)
}

fn get_formatted_diff<'a>(
    origin: &'a str,
    path: &'a Vec<String>,
    values: Option<&Value>,
) -> Result<ansi_term::ANSIString<'a>, Box<dyn Error>> {
    let diff_symbol = get_diff_symbol(origin)?;
    let diff_color = get_diff_color(origin)?;

    match values {
        Some(v) => Ok(diff_color.paint(format!("{} {:?} {}", diff_symbol, path, v))),
        None => Ok(diff_color.paint(format!("{} {:?}", diff_symbol, path))),
    }
}

fn print_missing_key(origin: &str, path: &Vec<String>) -> () {
    match get_formatted_diff(origin, path, None) {
        Ok(diff) => println!("{}", diff),
        Err(e) => println!("Error found when calculating diff: {}", e),
    }
}

fn print_changed_value(path: &Vec<String>, local: &Value, remote: &Value) -> () {
    match get_formatted_diff(LOCAL, path, Some(local)) {
        Ok(diff) => println!("{}", diff),
        Err(e) => println!("Error found when calculating diff: {}", e),
    }
    match get_formatted_diff(REMOTE, path, Some(remote)) {
        Ok(diff) => println!("{}", diff),
        Err(e) => println!("Error found when calculating diff: {}", e),
    }
}

fn print_missing_keys(json1: &Map<String, Value>, json2: &Map<String, Value>) {
    let missing_keys_local = find_missing_keys(json1, json2);

    for path in missing_keys_local {
        print_missing_key(LOCAL, &path);
    }

    let missing_keys_remote = find_missing_keys(json2, json1);

    for path in missing_keys_remote {
        print_missing_key(REMOTE, &path);
    }
}

fn print_changed_values(json1: &Map<String, Value>, json2: &Map<String, Value>) {
    let changed_values = find_changed_values(json1, json2);
    for (path, local_v, remote_v) in changed_values {
        print_changed_value(&path, &local_v, &remote_v);
    }
}

fn calculate_diff(json1: &Map<String, Value>, json2: &Map<String, Value>) -> () {
    print_missing_keys(json1, json2);
    print_changed_values(json1, json2);
}

// TODO better error handling when JSON file has invalid JSON
pub fn read_json(local_path: &str, remote_path: &str) -> Result<(), Box<dyn Error>> {
    let local_file = File::open(local_path)?;
    let local_reader = BufReader::new(local_file);

    let remote_file = File::open(remote_path)?;
    let remote_reader = BufReader::new(remote_file);

    let local_v: Value = serde_json::from_reader(local_reader)?;
    let remote_v: Value = serde_json::from_reader(remote_reader)?;

    local_v.as_object().and_then(|local_v| {
        remote_v
            .as_object()
            .map(|remote_v| calculate_diff(local_v, remote_v))
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_diff_symbol_test() {
        let local_diff_symbol = get_diff_symbol(LOCAL);
        assert_matches!(local_diff_symbol, Ok("-"));
        let remote_diff_symbol = get_diff_symbol(REMOTE);
        assert_matches!(remote_diff_symbol, Ok("+"));
        let invalid_diff_symbol = get_diff_symbol("invalid");
        let expected_err = "Err should have happened: Invalid origin! Must be either LOCAL or REMOTE.";
        assert!(invalid_diff_symbol.is_err(), expected_err);
    }

    #[test]
    fn get_diff_color_test() {
        let local_diff_color = get_diff_color(LOCAL);
        assert_matches!(local_diff_color, Ok(Red));
        let remote_diff_color = get_diff_color(REMOTE);
        assert_matches!(remote_diff_color, Ok(Green));
        let invalid_diff_color = get_diff_color("invalid");
        let expected_err = "Err should have happened: Invalid origin! Must be either LOCAL or REMOTE.";
        assert!(
            invalid_diff_color.is_err(),
            expected_err
        );
    }
}
