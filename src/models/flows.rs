use std::path::PathBuf;
use super::env_file::EnvironmentFile;

#[serde(rename_all = "kebab-case")]
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Flow {
    pub name: String,
    pub flow: Vec<String>,
    pub env_file: Option<EnvironmentFile>,
}

impl Flow {
    pub fn new() -> Flow {
        let flows =vec![
            String::from("pre"),String::from("unit-test"),String::from("build"),String::from("functional-test"),
            String::from("template"),String::from("deploy"),String::from("system-test"),String::from("post")
        ];
        
        Flow {
            name: String::from("default"),
            flow: flows,
            env_file: Some(EnvironmentFile::new()),
        }
    }
    
    pub fn collect_paths(&mut self) -> Vec<PathBuf> {
        let mut collection: Vec<PathBuf> = Vec::new();
        
        match &self.env_file {
            Some(env) => collection.extend(env.clone().collect_paths()),
            None => {},
        }
        
        collection
    }
}