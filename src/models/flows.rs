use super::env_file::EnvironmentFile;

#[serde(rename_all = "kebab-case")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flow {
    pub name: String,
    pub flow: Vec<String>,
    pub env_file: Option<EnvironmentFile>,
}
