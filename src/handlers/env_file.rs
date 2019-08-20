extern crate dotenv;
extern crate reqwest;
use crate::models::env_file::EnvironmentFile;

/// The EnvironmentFile load a dot env file. (eg: .env)
#[derive(Debug, Default)]
pub struct EnvironmentFileHandler {}

impl EnvironmentFileHandler {
    pub fn process(env_model: EnvironmentFile) {
        let env_file_type = env_model.clone().file_location_type();

        // load file from source(env_file_type)
        match env_file_type {
            Some(x) => {
                if x == "path" {
                    let env_run = dotenv::from_filename(&env_model.clone().path.unwrap());
                    match env_run {
                        Ok(_x) => println!("Loaded env-file: {:?}\n", env_model.path.unwrap()),
                        Err(e) => eprintln!("Cannot load env-file: {}\n", e),
                    }
                } else if x == "url" {
                    let response = reqwest::get(&env_model.url.unwrap());
                    match response {
                        Ok(mut y) => {
                            match y.text() {
                                Ok(_z) => {
                                    // write to file and run dotenv.
                                }
                                Err(_e) => {}
                            }
                        }
                        Err(_e) => {}
                    }
                }
            }
            None => {}
        }
    }
}
