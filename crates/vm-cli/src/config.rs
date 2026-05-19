use std::collections::HashMap;
use std::fs;

pub fn load_layer_config(path: &str) -> HashMap<String, bool> {
    let mut map = HashMap::new();
    let Ok(raw) = fs::read_to_string(path) else {
        return map;
    };

    for line in raw.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        if let Some((k, v)) = line.split_once('=') {
            if let Some(name) = k.strip_prefix("layer.") {
                map.insert(name.to_string(), v.trim().eq_ignore_ascii_case("on"));
            }
        }
    }
    map
}
