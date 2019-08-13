use super::env_prompt::EnvironmentPrompt;
use super::template::Template;

use std::path::PathBuf;
use std::collections::HashMap;

#[serde(rename_all = "kebab-case")]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Step {
    pub order: Option<usize>,
    pub env_prompt: Option<Vec<EnvironmentPrompt>>,
    pub script: Option<PathBuf>,
    pub template: Option<Vec<Template>>,
    pub post_script: Option<PathBuf>,
}

impl Step {
    pub fn new() -> Step {
        Step {
            order: None,
            env_prompt: Some(vec![EnvironmentPrompt::new()]),
            script: None,
            template: Some(vec![Template::new()]),
            post_script: None,
        }
    }
    
    pub fn default_set(self) -> HashMap<String, Step> {
        let mut steps = HashMap::new();
        let mut step = Step::new();
        let template = Template::new();
        step.template = Some(vec![template]);
        
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
