use std::path::PathBuf;
use super::Dockerization;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Containerization {
    pub docker: Option<Dockerization>,
}

impl Containerization {
    pub fn new() -> Self  {
        Default::default()
    }
    
    pub fn collect_paths(&mut self) -> Vec<PathBuf> {
        let mut collection: Vec<PathBuf> = Vec::new();
        
        match &self.docker {
            Some(env) => collection.extend(env.clone().collect_paths()),
            None => {},
        }
        
        collection
    }
}

