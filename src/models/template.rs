use std::path::PathBuf;

#[serde(rename_all = "kebab-case")]
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Template {
    pub name: String,
    pub template: Option<PathBuf>,
    pub out_file: Option<PathBuf>,
    pub params: Option<PathBuf>,
}

impl Template {
    pub fn new() -> Template {
        Default::default()
    }
}