use dotenv;
use clap::{ArgMatches};
use dialoguer::Input;
use std::env;

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
                    let mut prompt = Input::new();
                    if default_value.len() > 0 {
                        prompt.default(default_value.as_str().to_owned());
                    }
                    if value_options.len() > 0 {
                        let options = format!("( opts: {})",  value_options);
                        question.push_str(options.as_str());
                    }
                    let interact = prompt.with_prompt(&question.as_str()).interact().unwrap().to_owned();
                    env::set_var(name, interact);
                }
            }
        }
    }
}