extern crate chrono;
extern crate rustc_serialize;
use self::chrono::*;
use self::rustc_serialize::json::*;
use std::collections::BTreeMap;

pub fn generate(obj: &Object) -> String {
    let mut map = BTreeMap::new();
    for (key, value) in obj.iter() {
        let value_type = match Some(&*value.to_string()) {
            Some("\"string\"") => string_gen(),
            Some("\"date\"") => date_gen(),
            _ => "test".to_string()
        };
        map.insert(key,value_type.to_string());
    }
    let encoded = encode(&map).unwrap();
    encoded
}

fn string_gen() -> String {
    let data = String::from("Some generated value"); 
    data
}

fn date_gen() -> String {
    let utc: DateTime<UTC> = UTC::now();
    utc.to_string()
}
