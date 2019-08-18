extern crate serde_yaml;

use std::path::PathBuf;
use super::api_validate;
use super::mc_file;
use crate::models::{
    MasterOfCeremonyModelSelection, MasterOfCeremonyModel, MasterOfCeremonyFlowModel, MasterOfCeremonyTemplateModel, Step,
    MasterOfCeremonyPromptModel, MasterOfCeremonyRepositoryModel, MasterOfCeremonyEnvironmentFileModel, MasterOfCeremonyStepModel
};

/// The ModelHandler will intake a file as a String and populate the model fields.  Files currently
/// supported as string only include YAML at the moment but will soon handle JSON.
#[derive(Debug, Default, Clone)]
pub struct MasterOfCeremonyModelHandler {
    pub configs: Vec<PathBuf>,
    pub model_used: String,
    pub mc_model: MasterOfCeremonyModel,
    pub mc_env: MasterOfCeremonyEnvironmentFileModel,
    pub mc_flow: MasterOfCeremonyFlowModel,
    pub mc_prompt: MasterOfCeremonyPromptModel,
    pub mc_templates: MasterOfCeremonyTemplateModel,
    pub mc_repository: MasterOfCeremonyRepositoryModel,
    pub mc_steps: MasterOfCeremonyStepModel,
}

impl MasterOfCeremonyModelHandler {
    pub fn new() -> MasterOfCeremonyModelHandler {
        Default::default()
    }
    
    pub fn set_config_files(&mut self, config: &Option<PathBuf>) {
        let config_file_map = mc_file::MasterOfCeremonyFileHandler::get_config_paths(
            mc_file::MasterOfCeremonyFileHandler::new(), 
            config
        );
        
        self.configs = config_file_map.clone();
        for config in config_file_map {
            let f = std::fs::read_to_string(config.clone());
            match f {
                Ok(x) => self.set_api(&x),
                Err(_e) => {
                    std::process::exit(1)
                }
            }
        }
    }
    
    pub fn combine_api_models(&mut self) -> MasterOfCeremonyModel {
        if self.configs.len() > 1 {
            if self.mc_model.api.is_empty() { 
                self.mc_model.api = String::from("mc"); 
                self.mc_model.version = String::from("2.0"); }
            if self.mc_model.specs.env_file.is_none() { self.mc_model.specs.env_file = Some(self.mc_env.specs.to_owned()); }
            if self.mc_model.specs.env_prompt.is_none() { self.mc_model.specs.env_prompt = Some(self.mc_prompt.specs.to_owned()); }
            if self.mc_model.specs.flows.is_none() { self.mc_model.specs.flows = Some(self.mc_flow.specs.to_owned()); }
            if self.mc_model.specs.repository.is_none() { self.mc_model.specs.repository = Some(self.mc_repository.specs.to_owned()); }
            if self.mc_model.specs.steps.is_empty() { self.mc_model.specs.steps = self.mc_steps.specs.to_owned(); }
            if !self.mc_model.specs.steps.contains_key("template") { 
                let mut template_step = Step::new_empty();
                template_step.templates = Some(self.mc_templates.specs.to_owned());
                self.mc_model.specs.steps.insert(String::from("template"), template_step); 
            }
        }
        self.mc_model.clone()
    }
    
    pub fn get_api_version(&mut self, config: String) -> (String, String) {
        let config_fmt: serde_yaml::Value = serde_yaml::from_str(&config).unwrap();
        let api = (match config_fmt["api"].as_str() { Some(x) => {x}, 
            None => {eprintln!("Must assign api field.");""}}).to_string();
        let version = (match config_fmt["version"].as_str() { Some(x) => {x}, 
            None => {eprintln!("Must assign version field.");""}}).to_string();
        
        (api, version)
    }
    
    pub fn set_api(&mut self, yaml_file: &String) {
        let api_version = self.get_api_version(yaml_file.to_string());
        api_validate::ApiValidateHandler::valid_api_ver(api_version.0.to_owned(), api_version.1.to_owned());
        
        if api_version.0 == "mc-flows" {
            let model: MasterOfCeremonyFlowModel = serde_yaml::from_str(&yaml_file).unwrap();
            self.mc_flow = model;
            if self.model_used != "mc" {
                self.model_used = "mc-flows".to_owned();
            } else if self.configs.len() > 1 { self.model_used = "mc".to_owned(); }
        }  else if api_version.0 == "mc-env" {
            let model: MasterOfCeremonyEnvironmentFileModel = serde_yaml::from_str(&yaml_file).unwrap();
            self.mc_env = model;
            if self.model_used != "mc" {
                self.model_used = "mc-env".to_owned();
            } else if self.configs.len() > 1 { self.model_used = "mc".to_owned(); }
        }  else if api_version.0 == "mc-prompts" {
            let model: MasterOfCeremonyPromptModel = serde_yaml::from_str(&yaml_file).unwrap();
            self.mc_prompt = model;
            if self.model_used != "mc" {
                self.model_used = "mc-prompts".to_owned();
            } else if self.configs.len() > 1 { self.model_used = "mc".to_owned(); }
        }  else if api_version.0 == "mc-repo" {
            let model: MasterOfCeremonyRepositoryModel = serde_yaml::from_str(&yaml_file).unwrap();
            self.mc_repository = model;
            if self.model_used != "mc" {
                self.model_used = "mc-repo".to_owned();
            } else if self.configs.len() > 1 { self.model_used = "mc".to_owned(); }
        }  else if api_version.0 == "mc-templates" {
            let model: MasterOfCeremonyTemplateModel = serde_yaml::from_str(&yaml_file).unwrap();
            self.mc_templates = model;
            if self.model_used != "mc" {
                self.model_used = "mc-templates".to_owned();
            } else if self.configs.len() > 1 { self.model_used = "mc".to_owned(); }
        }  else if api_version.0 == "mc-steps" {
            let model: MasterOfCeremonyStepModel = serde_yaml::from_str(&yaml_file).unwrap();
            self.mc_steps = model;
            if self.model_used != "mc" {
                self.model_used = "mc-steps".to_owned();
            } else if self.configs.len() > 1 { self.model_used = "mc".to_owned(); }
        }  else if api_version.0 == "mc" {
            let model: MasterOfCeremonyModel = serde_yaml::from_str(&yaml_file).unwrap();
            self.mc_model = model;
            if self.configs.len() >= 1 {
                self.model_used = "mc".to_owned();
            }
        } 
    }
    
    pub fn get_api_type(&mut self) -> MasterOfCeremonyModelSelection {
        // Will need to comeback and make a hashmap for various api's and their versions.
        // Consider using true subversion check (https://crates.io/crates/semver)
        if self.configs.len() > 1 { self.combine_api_models(); }
        
        if self.model_used == "mc" || self.configs.len() > 1 {
            MasterOfCeremonyModelSelection::MasterOfCeremonyModel
        } else if self.model_used == "mc-flows" {
            MasterOfCeremonyModelSelection::MasterOfCeremonyFlowModel
        }  else if self.model_used == "mc-env" {
            MasterOfCeremonyModelSelection::MasterOfCeremonyEnvironmentFileModel
        }  else if self.model_used == "mc-prompts" {
            MasterOfCeremonyModelSelection::MasterOfCeremonyPromptModel
        }  else if self.model_used == "mc-repo" {
            MasterOfCeremonyModelSelection::MasterOfCeremonyRepositoryModel
        } else if self.model_used == "mc-templates" {
            MasterOfCeremonyModelSelection::MasterOfCeremonyTemplateModel
        } else if self.model_used == "mc-steps" {
            MasterOfCeremonyModelSelection::MasterOfCeremonyStepModel
        } else {
            MasterOfCeremonyModelSelection::None
        }
    }
}
