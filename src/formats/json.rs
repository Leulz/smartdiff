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

fn find_missing_keys(
    m1: &Map<String, Value>,
    m2: &Map<String, Value>,
    path: &Vec<String>,
    m1_origin: &str,
) -> () {
    let mut path_copy = path.to_vec();
    for (k, v) in m1 {
        path_copy.push(k.to_string());
        if !m2.contains_key(k) {
            print_missing_key(&m1_origin, &path_copy);
        } else if v.is_object() && m2[k].is_object() {
            find_missing_keys(
                m1[k].as_object().unwrap(),
                m2[k].as_object().unwrap(),
                &path_copy,
                m1_origin,
            );
        }
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

fn find_changed_values(
    local: &Map<String, Value>,
    remote: &Map<String, Value>,
    path: &Vec<String>,
) -> () {
    for (k, local_v) in local {
        if remote.contains_key(k) {
            let remote_v = &remote[k];
            if local_v.is_object() && remote_v.is_object() {
                let mut path_copy_nested = path.to_vec();
                path_copy_nested.push(k.to_string());
                find_changed_values(
                    local_v.as_object().unwrap(),
                    remote_v.as_object().unwrap(),
                    &path_copy_nested,
                );
            } else if *local_v != remote[k] {
                let mut path_copy = path.to_vec();
                path_copy.push(k.to_string());
                print_changed_value(&path_copy, local_v, &remote[k]);
            }
        }
    }
}

fn calculate_diff(json1: &Map<String, Value>, json2: &Map<String, Value>) -> () {
    find_missing_keys(json1, json2, &Vec::new(), LOCAL);
    find_missing_keys(json2, json1, &Vec::new(), REMOTE);
    find_changed_values(json1, json2, &Vec::new());
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
