use serde_json::map::Map;
use serde_json::Value;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use ansi_term::Colour::{Green, Red};

const LOCAL: &str = "local";
const REMOTE: &str = "remote";

fn get_diff_symbol(origin: &str) -> Result<&str, Box<dyn Error>> {
    let symbol = match origin {
        LOCAL => "-",
        REMOTE => "+",
        _ => panic!("Invalid origin! Must be either LOCAL or REMOTE."),
    };

    Ok(symbol)
}

fn get_diff_color(origin: &str) -> Result<ansi_term::Colour, Box<dyn Error>> {
    let color = match origin {
        LOCAL => Red,
        REMOTE => Green,
        _ => panic!("Invalid origin! Must be either LOCAL or REMOTE."),
    };

    Ok(color)
}

fn get_formatted_diff<'a>(
    origin: &'a str,
    path: &'a Vec<String>,
) -> Result<ansi_term::ANSIString<'a>, Box<dyn Error>> {
    let diff_symbol = get_diff_symbol(origin)?;
    let diff_color = get_diff_color(origin)?;

    Ok(diff_color.paint(format!("{} {:?}", diff_symbol, path)))
}

fn print_missing_key(origin: &str, path: &Vec<String>) -> () {
    match get_formatted_diff(origin, path) {
        Ok(diff) => println!("{}", diff),
        Err(e) => println!("Error found when calculating diff: {}", e),
    }
}

fn recur(
    m1: &Map<String, Value>,
    m2: &Map<String, Value>,
    path: &Vec<String>,
    m1_origin: &str,
) -> () {
    let mut path_copy = path.to_vec();
    for (k, v) in m1 {
        path_copy.push(k.to_string());
        if v.is_object() {
            if !m2.contains_key(k) {
                print_missing_key(&m1_origin, &path_copy);
            } else if !m2[k].is_object() {
                //TODO print_changed_type(...)
            } else {
                recur(
                    m1[k].as_object().unwrap(),
                    m2[k].as_object().unwrap(),
                    &path_copy,
                    m1_origin,
                );
            }
        } else {
            if !m2.contains_key(k) {
                print_missing_key(&m1_origin, &path_copy);
            } else {
                // TODO compare values in m1[k] and m2[k], if different print_changed_type(...)
            }
        }
    }
}

fn calculate_diff(json1: &Map<String, Value>, json2: &Map<String, Value>) -> () {
    recur(json1, json2, &Vec::new(), LOCAL);
    recur(json2, json1, &Vec::new(), REMOTE);
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
