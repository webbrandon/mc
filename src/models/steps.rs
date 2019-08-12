use super::env_prompt::EnvironmentPrompt;
use std::path::PathBuf;

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
        Default::default()
    }
}
