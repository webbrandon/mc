use dotenv;
use clap::{ArgMatches};
use dialoguer::{Confirmation, Input, Select};
use console::{Style};
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

fn ask_with_options(context: String, default_value: String, env_name: String, value_options: Vec<String>) -> String {
    let color_theme = ColorfulTheme::default();
    let question = if context.len() > 0 {context} else {format!("{} should be set?", env_name)};
    let response = Select::with_theme(&color_theme)
        .with_prompt(&question)
        .default(value_options.iter().position(|x| x == &default_value).unwrap_or(0))
        .items(&value_options[..])
        .interact()
        .unwrap();
    
    value_options[response].to_owned()
}

fn ask_bool(context: String, env_name: String) -> bool {
    let question = if context.len() > 0 {context} else {format!("{} should be set?", env_name)};
    let color_theme = ColorfulTheme {
        yes_style: Style::new().cyan(),
        no_style: Style::new().cyan(),
        ..ColorfulTheme::default()
    };
    Confirmation::with_theme(&color_theme)
            .with_text(&question)
            .interact()
            .unwrap()
}


fn ask_input(context: String, default_value: String, env_name: String, value_options: Vec<String>) -> String {
    let mut question = if context.len() > 0 {context} else {format!("Set value for {}", env_name)};
    let color_theme = ColorfulTheme::default();
    let mut prompt = Input::with_theme(&color_theme);
    
    if default_value.len() > 0 {
        let default_env = check_for_existing_value(&env_name);
        if default_env.len() > 0 {
            prompt.default(default_env.as_str().to_owned());
        } else {
            prompt.default(default_value.as_str().to_owned());
        }
    }
    if value_options.len() > 0 {
        let options = format!(".\nOptions: {:?}",  value_options);
        question.push_str(options.as_str());
    } else {
        question.push_str(":");
    }
    
    prompt.with_prompt(&question.as_str()).interact().unwrap().to_owned()
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
    pub fn load_env(env_file: String) {
        dotenv::from_filename(env_file.as_str()).ok();
    }
    pub fn run_env_prompt(env_array: &Vec<(String, String, String, String, Vec<String>)>, no_prompt: &bool) {
        if !no_prompt {
            for x in env_array {
                let (env_name, type_value, context, default_value, value_options) = x;
                if env_name.len() > 0 {
                    if type_value == "input" {
                        env::set_var(env_name, ask_input(context.to_string(), default_value.to_string(), env_name.to_string(), value_options.to_vec()))
                    } else if type_value == "option" {
                        env::set_var(env_name, ask_with_options(context.to_string(), default_value.to_string(), env_name.to_string(), value_options.to_vec()))
                    } else if type_value == "boolean" {
                        env::set_var(env_name, ask_bool(context.to_string(), env_name.to_string()).to_string())
                    }
                }
            }
        }
    }
}