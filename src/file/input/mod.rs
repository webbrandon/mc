use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use serde_json::value::Value as Json;
use file::bad_format_close_app;
use std::process;

pub fn file_to_string(file: &str) -> String {
    let mut s = String::new();
    match File::open(file) {
        Ok(mut a) => {
            a.read_to_string(&mut s).unwrap_or(0);
        },
        Err(_e) => {
            println!("Unable to open {}.", file);
            process::exit(0x0100);
        },
    }
    s
}

pub fn parse_json(text: &String) -> Json {
    match Json::from_str(text) {
        Ok(json) => json,
        Err(_) => bad_format_close_app(),
    }
}