use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct EnvironmentFile {
    pub url: Option<String>,
    pub path: Option<PathBuf>,
}

impl EnvironmentFile {
    pub fn new() -> EnvironmentFile {
        Default::default()
    }
    
    pub fn file_location_type(self) -> Option<&'static str> {
        if self.url.is_some() {
            Some("url")
        } else if self.path.is_some() {
            Some("path")
        } else {
            None
        }
    }
}
