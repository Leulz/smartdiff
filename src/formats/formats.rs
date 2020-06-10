use std::collections::HashSet;

pub fn get_supported_formats() -> HashSet<String> {
    let mut supported_formats : HashSet<String> = HashSet::new();
    supported_formats.insert("json".to_string());

    supported_formats
}
