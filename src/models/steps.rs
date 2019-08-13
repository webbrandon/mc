use super::env_prompt::EnvironmentPrompt;
use std::path::PathBuf;
use std::collections::HashMap;

#[serde(rename_all = "kebab-case")]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Step {
    pub order: Option<usize>,
    pub env_prompt: Option<Vec<EnvironmentPrompt>>,
    pub script: Option<PathBuf>,
    pub template: Option<PathBuf>,
    pub out_file: Option<PathBuf>,
    pub params: Option<PathBuf>,
    pub post_script: Option<PathBuf>,
}

impl Step {
    pub fn new() -> Step {
        Step {
            order: None,
            env_prompt: Some(vec![EnvironmentPrompt::new()]),
            script: None,
            template: None,
            out_file: None,
            params: None,
            post_script: None,
        }
    }
    
    pub fn default_set(self) -> HashMap<String, Step> {
        let mut steps = HashMap::new();
        let step = Step::new();
        
        steps.insert(String::from("pre"), step.clone());
        steps.insert(String::from("unit-test"), step.clone());
        steps.insert(String::from("build"), step.clone());
        steps.insert(String::from("template"), step.clone());
        steps.insert(String::from("deploy"), step.clone());
        steps.insert(String::from("functional-test"), step.clone());
        steps.insert(String::from("system-test"), step.clone());
        steps.insert(String::from("post"), step.clone());
        
        steps
    }
    
    pub fn new_empty() -> Step {
        Default::default()
    }
}
