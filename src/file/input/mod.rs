use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use serde_json::value::Value as Json;
use file::bad_format_close_app;

pub fn file_to_string(file: &str) -> String {
    let mut f = File::open(file).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap_or(0);
    s
}

pub fn parse_json(text: &String) -> Json {
    match Json::from_str(text) {
        Ok(json) => json,
        Err(_) => bad_format_close_app(),
    }
}