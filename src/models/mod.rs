pub mod details;
pub mod env_file;
pub mod env_prompt;
pub mod flows;
pub mod repository;
pub mod specs;
pub mod steps;

pub use env_file::EnvironmentFile;
pub use env_prompt::EnvironmentPrompt;
pub use repository::Repository;
pub use details::Details;
pub use flows::Flow;
pub use specs::Specs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MasterOfCeremonyModelSelection {
    MasterOfCeremonyModel,
    MasterOfCeremonyPromptModel,
    MasterOfCeremonyFlowModel,
    MasterOfCeremonyRepositoryModel,
    MasterOfCeremonyEnvironmentFileModel,
    None
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MasterOfCeremonyModel {
    pub api: String,
    pub version: String,
    pub metadata: Option<Details>,
    pub specs: Specs,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MasterOfCeremonyPromptModel {
    pub api: String,
    pub version: String,
    pub metadata: Option<Details>,
    pub specs: Vec<EnvironmentPrompt>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MasterOfCeremonyRepositoryModel {
    pub api: String,
    pub version: String,
    pub metadata: Option<Details>,
    pub specs: Repository,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MasterOfCeremonyFlowModel {
    pub api: String,
    pub version: String,
    pub metadata: Option<Details>,
    pub specs: Vec<Flow>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MasterOfCeremonyEnvironmentFileModel {
    pub api: String,
    pub version: String,
    pub metadata: Option<Details>,
    pub specs: EnvironmentFile,
}
