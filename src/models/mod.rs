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

/// Master Of Cermony API selection.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MasterOfCeremonyModelSelection {
    MasterOfCeremonyModel,
    MasterOfCeremonyPromptModel,
    MasterOfCeremonyFlowModel,
    MasterOfCeremonyRepositoryModel,
    MasterOfCeremonyEnvironmentFileModel,
}

/// Master Of Cermony API Schema 2.0
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MasterOfCeremonyModel {
    pub api: String,
    pub version: String,
    pub metadata: Option<Details>,
    pub specs: Specs,
}

/// Master Of Cermony Prompt API Schema 2.0
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MasterOfCeremonyPromptModel {
    pub api: String,
    pub version: String,
    pub metadata: Option<Details>,
    pub specs: Vec<EnvironmentPrompt>,
}

/// Master Of Cermony Repository API Schema 2.0
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MasterOfCeremonyRepositoryModel {
    pub api: String,
    pub version: String,
    pub metadata: Option<Details>,
    pub specs: Repository,
}

/// Master Of Cermony Flow API Schema 2.0
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MasterOfCeremonyFlowModel {
    pub api: String,
    pub version: String,
    pub metadata: Option<Details>,
    pub specs: Vec<Flow>,
}

/// Master Of Cermony Environment File API Schema 2.0
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MasterOfCeremonyEnvironmentFileModel {
    pub api: String,
    pub version: String,
    pub metadata: Option<Details>,
    pub specs: EnvironmentFile,
}

impl MasterOfCeremonyModel {
    pub fn new() -> MasterOfCeremonyModel {
        MasterOfCeremonyModel {
            api: String::from("mc"),
            version: String::from("v2.0"),
            metadata: Some(Details::new()),
            specs: Specs::new(),
        }
    }
}

/// Master Of Cermony API Schema 2.0
impl MasterOfCeremonyPromptModel {
    pub fn new() -> MasterOfCeremonyPromptModel {
        MasterOfCeremonyPromptModel {
            api: String::from("mc-prompt"),
            version: String::from("v2.0"),
            metadata: Some(Details::new()),
            specs: vec![EnvironmentPrompt::new()],
        }
    }
}

impl MasterOfCeremonyRepositoryModel {
    pub fn new() -> MasterOfCeremonyRepositoryModel {
        MasterOfCeremonyRepositoryModel {
            api: String::from("mc-repo"),
            version: String::from("v2.0"),
            metadata: Some(Details::new()),
            specs: Repository::new(),
        }
    }
}

impl MasterOfCeremonyFlowModel {
    pub fn new() -> MasterOfCeremonyFlowModel {
        MasterOfCeremonyFlowModel {
            api: String::from("mc-flows"),
            version: String::from("v2.0"),
            metadata: Some(Details::new()),
            specs: vec![Flow::new()],
        }
    }
}

impl MasterOfCeremonyEnvironmentFileModel {
    pub fn new() -> MasterOfCeremonyEnvironmentFileModel {
        MasterOfCeremonyEnvironmentFileModel {
            api: String::from("mc-env"),
            version: String::from("v2.0"),
            metadata: Some(Details::new()),
            specs: EnvironmentFile::new(),
        }
    }
}
