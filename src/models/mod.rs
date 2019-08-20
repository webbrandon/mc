pub mod details;
pub mod env_file;
pub mod env_prompt;
pub mod flows;
pub mod repository;
pub mod specs;
pub mod steps;
pub mod template;
pub mod containerization;
pub mod dockerization;

use std::collections::HashMap;
pub use containerization::Containerization;
pub use dockerization::Dockerization;
pub use env_file::EnvironmentFile;
pub use env_prompt::EnvironmentPrompt;
pub use template::Template;
pub use repository::Repository;
pub use details::Details;
pub use flows::Flow;
pub use specs::Specs;
pub use steps::Step;

/// Master Of Cermony API selection.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MasterOfCeremonyModelSelection {
    MasterOfCeremonyModel,
    MasterOfCeremonyPromptModel,
    MasterOfCeremonyFlowModel,
    MasterOfCeremonyRepositoryModel,
    MasterOfCeremonyEnvironmentFileModel,
    MasterOfCeremonyTemplateModel,
    MasterOfCeremonyStepModel,
    MasterOfCeremonyContainerizationModel,
    None
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

/// Master Of Cermony Environment File API Schema 2.0
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MasterOfCeremonyTemplateModel {
    pub api: String,
    pub version: String,
    pub metadata: Option<Details>,
    pub specs: Vec<Template>,
}

/// Master Of Cermony Step API Schema 2.0
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MasterOfCeremonyStepModel {
    pub api: String,
    pub version: String,
    pub metadata: Option<Details>,
    pub specs: HashMap<String, Step>,
}

/// Master Of Cermony Containerization API Schema 2.0
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MasterOfCeremonyContainerizationModel {
    pub api: String,
    pub version: String,
    pub metadata: Option<Details>,
    pub specs: Containerization,
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
    
    pub fn new_empty() -> MasterOfCeremonyModel {
        Default::default()
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

impl MasterOfCeremonyTemplateModel {
    pub fn new() -> MasterOfCeremonyTemplateModel {
        MasterOfCeremonyTemplateModel {
            api: String::from("mc-templates"),
            version: String::from("v2.0"),
            metadata: Some(Details::new()),
            specs: vec![Template::new()],
        }
    }
}

/// Master Of Cermony Containerization API Schema 2.0
impl MasterOfCeremonyContainerizationModel {
    pub fn new() -> MasterOfCeremonyContainerizationModel {
        MasterOfCeremonyContainerizationModel {
            api: String::from("mc-container"),
            version: String::from("v1.0"),
            metadata: Some(Details::new()),
            specs: Containerization::new(),
        }
    }
}

impl MasterOfCeremonyStepModel {
    pub fn new() -> MasterOfCeremonyStepModel {
        let mut step_model: HashMap<String, Step> = HashMap::new();
        step_model.insert(String::from(""), Step::new());
        MasterOfCeremonyStepModel {
            api: String::from("mc-steps"),
            version: String::from("v2.0"),
            metadata: Some(Details::new()),
            specs: step_model,
        }
    }
    
    pub fn default_sort(&mut self) -> Vec<(String, Step)> {
        let mut sorted_steps: Vec<(String, Step)> = Vec::new();
        let mut steps_copy = self.specs.clone();
        let step_order_instructions = vec![
            "pre",
            "unit-test",
            "build",
            "functional-test",
            "template",
            "deploy",
            "system-test",
            "post",
        ];

        for step_order in step_order_instructions {
            if steps_copy.contains_key(step_order) {
                match steps_copy.get(step_order) {
                    Some(x) => {
                        sorted_steps.push((step_order.to_string(), x.clone()));
                        steps_copy.remove(step_order);
                    }
                    None => {}
                }
            }
        }

        //  Order steps by our defined default pipeline order.  Track custom step request for later.
        let custom_steps: Vec<(String, Step)> = steps_copy.clone().into_iter().collect();

        // Add custom step elements.  Insert by order if requested and will be based on
        // available defined steps in a default pipeline order or else push to end in order
        // recieved.  We should rethink this structure as custom event will more than likely
        // always be random due to the nature of HashMap.
        for step_model in custom_steps {
            match step_model.1.order {
                Some(order) => {
                    if sorted_steps.len() > order {
                        sorted_steps.insert(order - 1, step_model);
                    }
                }
                None => {
                    sorted_steps.push(step_model);
                }
            }
        }

        sorted_steps
    }
}
