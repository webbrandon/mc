use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::handlers::env_prompt::EnvironmentPromptHandler;
use crate::models;
use serde_yaml;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct MasterOfCeremonyApiCreator { }

impl MasterOfCeremonyApiCreator {
    pub fn new() -> MasterOfCeremonyApiCreator {
        Default::default()
    }
    
    fn write_api_to_file(self, path: &Path, api_config: String) {
        let mut file: std::fs::File;
        if !path.exists() {
            file = match File::create(path) {
                Err(why) => panic!("Couldn't create {}: {}", path.display(), why.description()),
                Ok(file) => file,
            };
        } else {
            let question = format!("Delete existing \"{}\" file?", path.display());
            if EnvironmentPromptHandler::ask_bool(&question) {
                file = match File::create(path) {
                    Err(why) => panic!("Couldn't create {}: {}", path.display(), why.description()),
                    Ok(file) => file,
                };
            } else {
                std::process::exit(0)
            }
        }

        match file.write_all(api_config.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", path.display(), why.description()),
            Ok(_) => println!("Successfully wrote to {}", path.display()),
        }
    }
    
    pub fn generate_blank_template(self, api: String) {
        if api == "mc-flows" {
            self.write_api_to_file(Path::new("mc-flows.yaml"), serde_yaml::to_string(&models::MasterOfCeremonyFlowModel::new()).unwrap());
        } else if api == "mc-env" {
            self.write_api_to_file(Path::new("mc-env.yaml"), serde_yaml::to_string(&models::MasterOfCeremonyEnvironmentFileModel::new()).unwrap());
        } else if api == "mc-prompts" {
            self.write_api_to_file(Path::new("mc-prompts.yaml"), serde_yaml::to_string(&models::MasterOfCeremonyPromptModel::new()).unwrap());
        } else if api == "mc-repo" {
            self.write_api_to_file(Path::new("mc-repo.yaml"), serde_yaml::to_string(&models::MasterOfCeremonyRepositoryModel::new()).unwrap());
        } else if api == "mc-templates" {
            self.write_api_to_file(Path::new("mc-templates.yaml"), serde_yaml::to_string(&models::MasterOfCeremonyTemplateModel::new()).unwrap());
        } else if api == "mc-steps" {
            self.write_api_to_file(Path::new("mc-steps.yaml"), serde_yaml::to_string(&models::MasterOfCeremonyStepModel::new()).unwrap());
        } else {
            self.write_api_to_file(Path::new("mc.yaml"), serde_yaml::to_string(&models::MasterOfCeremonyModel::new()).unwrap());
        }
    }
    
    pub fn create_api_config(self, api: String, guide: bool) {
        if guide {
            println!("Coming Soon");
        } else {
            self.generate_blank_template(api);
        }
    }
}