use dotenv;
use clap::{ArgMatches};
use dialoguer::Input;
use std::env;

use dialoguer::{theme::ColorfulTheme};

fn check_for_existing_value(value_name: &String) -> String {
    match env::var(value_name) {
        Ok(val) => {
            val
        },
        Err(_e) => {
            // println!("{}", _e);
            String::new()
        },
    }
}

pub struct EnvFile {}

impl EnvFile {
    pub fn check_and_load(matches: &ArgMatches) {
        let has_env_file = matches.is_present("env");
        if has_env_file {
            let file_path = matches.value_of("env").unwrap_or("").to_owned();
            dotenv::from_filename(file_path.as_str()).ok();
        }
    }
    pub fn run_env_prompt(env_array: &Vec<(String, String, String)>, no_prompt: &bool) {
        if !no_prompt {
            for x in env_array {
                let (name, default_value, value_options) = x;
                if name.len() > 0 {
                    let mut question = format!("Set value for {}", name);
                    let color_theme = ColorfulTheme::default();
                    let mut prompt = Input::with_theme(&color_theme);
                    if default_value.len() > 0 {
                        let default_env = check_for_existing_value(name);
                        if default_env.len() > 0 {
                            prompt.default(default_env.as_str().to_owned());
                        } else {
                            prompt.default(default_value.as_str().to_owned());
                        }
                    }
                    if value_options.len() > 0 {
                        let options = format!(".\nOptions: {}",  value_options);
                        question.push_str(options.as_str());
                    } else {
                        question.push_str(":");
                    }
                    let interact = prompt.with_prompt(&question.as_str()).interact().unwrap().to_owned();
                    env::set_var(name, interact);
                }
            }
        }
    }
}