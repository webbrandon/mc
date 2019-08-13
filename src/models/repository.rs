use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Repository {
    pub url: Option<String>,
    pub path: Option<PathBuf>,
}
