use dotenv;
use clap::{ArgMatches};

pub struct EnvFile {}

impl EnvFile {
    pub fn check_and_load(matches: &ArgMatches) {
        let has_env_file = matches.is_present("env");
        if has_env_file {
            let file_path = matches.value_of("env").unwrap_or("").to_owned();
            dotenv::from_filename(file_path.as_str()).ok();  
        }
    }    
}