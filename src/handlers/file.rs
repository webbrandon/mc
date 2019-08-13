use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use super::env_prompt::EnvironmentPromptHandler;
extern crate glob;
use glob::glob;

pub fn load(file: PathBuf) -> String {
    match read_into_string(file) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1)
        }
    }
}

pub fn read_into_string(file: PathBuf) -> Result<String, String> {
    let f = fs::read_to_string(file.clone());
    match f {
        Ok(x) => Ok(x),
        Err(e) => {
            Err(format!("Failed to open file: {:?}\nError: {}", file.to_str().unwrap(), e))
        }
    }
}

pub fn load_config(file: &Option<PathBuf>, prompt: bool) -> String {
    match file {
        Some(x) => load(x.to_path_buf()),
        None => {
            let mut configs: Vec<String> = Vec::new();
            for e in glob("./mc*.yaml").expect("") {
                configs.push(e.unwrap().display().to_string());
            }
            for e in glob("./*.mc.yaml").expect("") {
                configs.push(e.unwrap().display().to_string());
            }
            for e in glob("./mc.yml").expect("") {
                configs.push(e.unwrap().display().to_string());
            }
            for e in glob("./*.mc.yml").expect("") {
                configs.push(e.unwrap().display().to_string());
            }
            for e in glob("./mc*.yml").expect("") {
                configs.push(e.unwrap().display().to_string());
            }
            configs.push("Exit".to_string());
            
            if configs.len() == 1 {
                std::process::exit(1)
            }
            
            if configs.len() == 2 {
                load(Path::new(&configs[0]).to_path_buf())
            } else {
                if prompt {
                    let file_to_load = EnvironmentPromptHandler::ask_with_options(&"You have several mc api configs to load. Please choose:".to_string(), &None, &Some(configs));
                    if file_to_load == "Exit" {
                        std::process::exit(1)
                    } else {
                        load(Path::new(&file_to_load).to_path_buf())
                    }
                } else {
                    std::process::exit(1)
                }
            }
        },
    }
}

pub fn out(filepath: PathBuf, text: &String) {
    let f = File::create(filepath).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(text.as_bytes()).expect("Unable to write data");
}
