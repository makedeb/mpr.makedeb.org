use std::collections::HashMap;
use serde_json::{json, Value};

// Get the default context for templates.
pub fn get_context<'a>() -> HashMap<&'a str, Value> {
    let mut map: HashMap<&str, Value> = HashMap::new();
    map.insert("mpr", json!({
        "makedeb_url": "makedeb.org",
        "hw_url": "hunterwittenborn.com"
    }));

    map
}