#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct EnvironmentPrompt {
    pub name: String,
    pub kind: Option<String>,
    pub default: Option<String>,
    pub context: Option<String>,
    pub options: Option<Vec<String>>,
}

impl EnvironmentPrompt {
    pub fn new() -> EnvironmentPrompt {
        Default::default()
    }
}
