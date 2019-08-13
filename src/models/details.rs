#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Details {
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub version: Option<String>,
}

impl Details {
    pub fn new() -> Details {
        Default::default()
    }
}
