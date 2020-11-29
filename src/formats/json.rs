use serde_json::map::Map;
use serde_json::Value;

fn _find_missing_keys<'a>(
    m1: &'a Map<String, Value>,
    m2: &'a Map<String, Value>,
    path: &'a Vec<String>,
) -> Vec<Vec<String>> {
    let mut diff = Vec::new();
    for (k, v) in m1 {
        let mut path_copy = path.to_vec();
        path_copy.push(k.to_string());
        if !m2.contains_key(k) {
            diff.push(path_copy.clone());
        } else if v.is_object() && m2[k].is_object() {
            let level_diff = _find_missing_keys(
                m1[k].as_object().unwrap(),
                m2[k].as_object().unwrap(),
                &path_copy,
            );
            diff.extend(level_diff);
        }
    }
    diff
}

pub fn find_missing_keys<'a>(
    m1: &'a Map<String, Value>,
    m2: &'a Map<String, Value>,
) -> Vec<Vec<String>> {
    _find_missing_keys(&m1, &m2, &Vec::new())
}

fn _find_changed_values<'a>(
    local: &'a Map<String, Value>,
    remote: &'a Map<String, Value>,
    path: &'a Vec<String>,
) -> Vec<(Vec<String>, Value, Value)> {
    let mut diff = Vec::new();
    for (k, local_v) in local {
        if remote.contains_key(k) {
            let remote_v = &remote[k];
            if local_v.is_object() && remote_v.is_object() {
                let mut path_copy_nested = path.to_vec();
                path_copy_nested.push(k.to_string());
                let level_diff = _find_changed_values(
                            local_v.as_object().unwrap(),
                            remote_v.as_object().unwrap(),
                            &path_copy_nested,
                        );
                diff.extend(level_diff);
            } else if *local_v != remote[k] {
                let mut path_copy = path.to_vec();
                path_copy.push(k.to_string());
                diff.push((path_copy.clone(), local_v.clone(), remote_v.clone()));
            }
        }
    }

    diff
}

pub fn find_changed_values<'a>(
    local: &'a Map<String, Value>,
    remote: &'a Map<String, Value>
) -> Vec<(Vec<String>, Value, Value)> {
    _find_changed_values(&local, &remote, &Vec::new())
}
