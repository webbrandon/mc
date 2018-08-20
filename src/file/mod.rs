use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::io::{self, BufWriter, Write};
use std::str::FromStr;
use serde_json::value::Value as Json;

pub fn file_to_string(file: &str) -> String {
    let mut f = File::open(file).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap_or(0);
    s
}

pub fn bad_format_close_app() -> ! {
    writeln!(
        &mut io::stderr(),
        "{}",
        r#"Format of files is not accepted."#
    ).ok();
    process::exit(1);
}

pub fn parse_json(text: &String) -> Json {
    match Json::from_str(text) {
        Ok(json) => json,
        Err(_) => bad_format_close_app(),
    }
}

pub fn outfile(filepath: &String, text: &String) {
    let f = File::create(filepath).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(text.as_bytes()).expect("Unable to write data");
}

pub fn out_term(text: &String) {
    writeln!(
        &mut io::stdout(),
        "{}",
        text
    ).ok();
}