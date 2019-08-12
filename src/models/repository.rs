use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repository {
    pub url: Option<String>,
    pub path: Option<PathBuf>,
}
