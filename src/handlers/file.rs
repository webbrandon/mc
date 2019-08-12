use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

pub fn load(file: PathBuf) -> String {
    let f = fs::read_to_string(file.clone());
    match f {
        Ok(x) => x,
        Err(e) => {
            eprintln!(
                "Failed to open file: {:?}\nError: {}",
                file.to_str().unwrap(),
                e
            );
            std::process::exit(1);
        }
    }
}

pub fn load_config(file: &Option<PathBuf>) -> String {
    match file {
        Some(x) => load(x.to_path_buf()),
        None => load(Path::new("./mc.yaml").to_path_buf()),
    }
}

pub fn out(filepath: PathBuf, text: &String) {
    let f = File::create(filepath).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(text.as_bytes()).expect("Unable to write data");
}
