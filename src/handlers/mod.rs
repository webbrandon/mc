mod cli_filters;
mod env_file;
mod env_prompt;
pub mod file;
mod model;
mod repository;
pub mod script;
mod steps;
pub mod template;
use crate::cli::Opt;
use crate::models::MasterOfCeremonyModel;
use model::MasterOfCeremonyModelHandler;
use std::path::PathBuf;
use steps::StepsHandler;

#[derive(Debug, Default, Clone)]
pub struct MasterOfCeremonyHandler {
    mute: bool,
    prompt: bool,
    data: MasterOfCeremonyModel,
}

impl MasterOfCeremonyHandler {
    pub fn new() -> MasterOfCeremonyHandler {
        Default::default()
    }

    pub fn process(&mut self) {
        // Step One: Run EnvironmentFile.
        self.environment_values_from_file();
        // Step Two: Run EnvironmentPrompt
        self.environment_values_from_prompt();
        // Step Three: Run Repository request.
        self.build_from_repository();
        // Step Four: Run Steps with Flow.
        self.process_steps();
    }

    /// Create MasterOfCeremonyHandler with a configuration file.
    pub fn from_file(file: &Option<PathBuf>) -> MasterOfCeremonyHandler {
        let configuration = file::load_config(file);
        let filtered_data = MasterOfCeremonyModelHandler::from_file(configuration);

        MasterOfCeremonyHandler {
            mute: false,
            prompt: true,
            data: filtered_data,
        }
    }

    /// Add StructOpt commandline arguments to MasterOfCeremonyModel.  Commandline option
    /// take priority over configuration file settings.
    pub fn add_cli(&mut self, cli_settings: Opt) -> &MasterOfCeremonyHandler {
        let mut handler = cli_filters::CliFiltersHandler::new();
        self.mute = cli_settings.mute;
        self.prompt = !cli_settings.prompt;

        handler.cli = cli_settings;
        handler.process(self);

        self
    }

    pub fn environment_values_from_file(&mut self) {
        // process top level env-file request.  Process even when flow defines env-file.
        match &self.data.specs.env_file {
            Some(x) => {
                env_file::EnvironmentFileHandler::process(x.clone());
            }
            None => {}
        }
        match &self.data.specs.flows {
            Some(y) => match &y[0].env_file {
                Some(z) => {
                    env_file::EnvironmentFileHandler::process(z.to_owned());
                }
                None => {}
            },
            None => {}
        }
    }

    pub fn environment_values_from_prompt(&mut self) {
        match &self.data.specs.env_prompt {
            Some(x) => {
                env_prompt::EnvironmentPromptHandler::process(x);
            }
            None => {}
        }
    }

    pub fn build_from_repository(&mut self) {
        match &self.data.specs.repository {
            Some(x) => {
                repository::RepositoryHandler::process(x.clone());
            }
            None => {}
        }
    }

    pub fn process_steps(&mut self) {
        let mut handler = StepsHandler::new();
        handler.process(self.data.specs.sort_steps(), self.prompt, self.mute);
    }
}
