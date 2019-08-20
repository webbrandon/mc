pub mod cli_filters;
pub mod env_file;
pub mod env_prompt;
pub mod api_validate;
pub mod model;
pub mod repository;
pub mod script;
pub mod steps;
pub mod template;
use std::path::PathBuf;

use crate::mc_file;
use crate::models::{MasterOfCeremonyModelSelection, Repository, EnvironmentFile, EnvironmentPrompt, Flow, Step};
use crate::cli::Opt;

use cli_filters::CliFiltersHandler;
use model::MasterOfCeremonyModelHandler;
use steps::StepsHandler;

/// Master Of Cermony handler for processing the various apis.
#[derive(Debug, Default, Clone)]
pub struct MasterOfCeremonyHandler {
    mute: bool,
    prompt: bool,
    cli: CliFiltersHandler,
    data: MasterOfCeremonyModelHandler,
}

impl MasterOfCeremonyHandler {
    pub fn new() -> MasterOfCeremonyHandler {
        Default::default()
    }

    /// Process the selected (mc.yaml) or/and defined (cli) api request.
    pub fn process(mut self) {
        match self.data.get_api_type() {
            MasterOfCeremonyModelSelection::MasterOfCeremonyModel => {
                let mut cli_filter = self.cli.clone();
                self = cli_filter.process(&mut self);
                self.build_from_repository(self.data.mc_model.specs.repository.clone());
                self.environment_values_from_file(self.data.mc_model.specs.env_file.clone());
                self.flow_environment_values_from_file(self.data.mc_model.specs.flows.clone());
                self.environment_values_from_prompt(self.data.mc_model.specs.env_prompt.clone());
                self.process_steps(self.data.mc_model.specs.clone().sort_steps().clone());
            },
            MasterOfCeremonyModelSelection::MasterOfCeremonyPromptModel => {
                self.environment_values_from_prompt(Some(self.data.mc_prompt.specs.clone()));
            },
            MasterOfCeremonyModelSelection::MasterOfCeremonyFlowModel => {
                self.flow_environment_values_from_file(Some(self.data.mc_flow.specs.clone()));
            },
            MasterOfCeremonyModelSelection::MasterOfCeremonyEnvironmentFileModel => {
                self.environment_values_from_file(Some(self.data.mc_env.specs.clone()));
            },
            MasterOfCeremonyModelSelection::MasterOfCeremonyRepositoryModel => {
                self.build_from_repository(Some(self.data.mc_repository.specs.clone()));
            },
            MasterOfCeremonyModelSelection::MasterOfCeremonyStepModel => {
                self.process_steps(self.data.clone().mc_steps.default_sort());
            },
            MasterOfCeremonyModelSelection::MasterOfCeremonyTemplateModel => {
                let mut handler = template::TemplateHandler::new();
                handler.template = self.data.mc_templates.specs;
                handler.mute = self.cli.cli.mute;
                handler.process();
            },
            MasterOfCeremonyModelSelection::None => eprintln!("Unrecognized API")
        }
    }

    /// Load api configuration file request.
    pub fn load_file(&mut self, file: &Option<PathBuf>) {
        let mut handler = MasterOfCeremonyModelHandler::new();
        handler.set_config_files(file);
        
        self.data = handler;
    }

    /// Add StructOpt commandline arguments to MasterOfCeremonyModel.  Commandline option
    /// take priority over configuration file settings.
    pub fn add_cli(&mut self, cli_settings: Opt) {
        let mut handler = cli_filters::CliFiltersHandler::new();
        self.mute = cli_settings.mute;
        self.prompt = !cli_settings.prompt;

        handler.cli = cli_settings;
        self.cli = handler;
    }

    pub fn environment_values_from_file(&mut self, env_file: Option<EnvironmentFile>) {
        // process top level env-file request.  Process even when flow defines env-file.
        match &env_file {
            Some(x) => {
                env_file::EnvironmentFileHandler::process(x.clone());
            }
            None => {}
        }
        match &self.data.mc_model.specs.flows {
            Some(y) => match &y[0].env_file {
                Some(z) => {
                    env_file::EnvironmentFileHandler::process(z.to_owned());
                }
                None => {}
            },
            None => {}
        }
    }

    pub fn flow_environment_values_from_file(&mut self, flow: Option<Vec<Flow>>) {
        match &flow {
            Some(y) => match &y[0].env_file {
                Some(z) => {
                    env_file::EnvironmentFileHandler::process(z.to_owned());
                }
                None => {}
            },
            None => {}
        }
    }

    pub fn environment_values_from_prompt(&mut self, env_prompt: Option<Vec<EnvironmentPrompt>>) {
        match &env_prompt {
            Some(x) => {
                env_prompt::EnvironmentPromptHandler::process(x);
            }
            None => {}
        }
    }

    pub fn build_from_repository(&mut self, repo: Option<Repository>) {
        match &repo {
            Some(x) => {
                repository::RepositoryHandler::process(x.clone());
            }
            None => {}
        }
    }

    pub fn process_steps(&mut self, steps: Vec<(String, Step)>) {
        let handler = StepsHandler::new();
        handler.process(steps, self.prompt, self.mute);
    }
}