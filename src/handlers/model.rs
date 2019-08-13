extern crate serde_yaml;

use super::api_validate;
use crate::models::{
    MasterOfCeremonyModelSelection, MasterOfCeremonyModel, MasterOfCeremonyFlowModel, 
    MasterOfCeremonyPromptModel, MasterOfCeremonyRepositoryModel, MasterOfCeremonyEnvironmentFileModel
};

/// The ModelHandler will intake a file as a String and populate the model fields.  Files currently
/// supported as string only include YAML at the moment but will soon handle JSON.
#[derive(Debug, Default, Clone)]
pub struct MasterOfCeremonyModelHandler {
    pub api: String,
    pub version: String,
    pub config_raw: String,
    pub mc_model: MasterOfCeremonyModel,
    pub mc_env: MasterOfCeremonyEnvironmentFileModel,
    pub mc_flow: MasterOfCeremonyFlowModel,
    pub mc_prompt: MasterOfCeremonyPromptModel,
    pub mc_repository: MasterOfCeremonyRepositoryModel,
}

impl MasterOfCeremonyModelHandler {
    pub fn new() -> MasterOfCeremonyModelHandler {
        Default::default()
    }
    
    pub fn process(&mut self) {
        if self.api == "mc" {
            let model: MasterOfCeremonyModel = serde_yaml::from_str(&self.config_raw).unwrap();
            self.mc_model = model;
        } else if self.api == "mc-flows" {
            let model: MasterOfCeremonyFlowModel = serde_yaml::from_str(&self.config_raw).unwrap();
            self.mc_flow = model;
        }  else if self.api == "mc-env" {
            let model: MasterOfCeremonyEnvironmentFileModel = serde_yaml::from_str(&self.config_raw).unwrap();
            self.mc_env = model;
        }  else if self.api == "mc-prompt" {
            let model: MasterOfCeremonyPromptModel = serde_yaml::from_str(&self.config_raw).unwrap();
            self.mc_prompt = model;
        }  else if self.api == "mc-repository" {
            let model: MasterOfCeremonyRepositoryModel = serde_yaml::from_str(&self.config_raw).unwrap();
            self.mc_repository = model;
        }
    }
    
    pub fn set_api(&mut self, yaml_file: String) {
        // Validate the YAML file structure.  
        // Exit on invalid configuration, this is serde default.
        let yaml_fmt: serde_yaml::Value = serde_yaml::from_str(&yaml_file).unwrap();
        
        self.config_raw = yaml_file;
        self.api = (match yaml_fmt["api"].as_str() { Some(x) => {x}, 
            None => {eprintln!("Must assign api field.");""}}).to_string();
        self.version = (match yaml_fmt["version"].as_str() { Some(x) => {x}, 
            None => {eprintln!("Must assign version field.");""}}).to_string();
        api_validate::ApiValidateHandler::valid_api_ver(self.api.clone(), self.version.clone());
    }
    
    pub fn get_api_type(&mut self) -> MasterOfCeremonyModelSelection {
        // Will need to comeback and make a hashmap for various api's and their versions.
        // Consider using true subversion check (https://crates.io/crates/semver)
        if self.api == "mc" {
            MasterOfCeremonyModelSelection::MasterOfCeremonyModel
        } else if self.api == "mc-flows" {
            MasterOfCeremonyModelSelection::MasterOfCeremonyFlowModel
        }  else if self.api == "mc-env" {
            MasterOfCeremonyModelSelection::MasterOfCeremonyEnvironmentFileModel
        }  else if self.api == "mc-prompt" {
            MasterOfCeremonyModelSelection::MasterOfCeremonyPromptModel
        }  else if self.api == "mc-repository" {
            MasterOfCeremonyModelSelection::MasterOfCeremonyRepositoryModel
        } else {
            MasterOfCeremonyModelSelection::None
        }
    }
}
