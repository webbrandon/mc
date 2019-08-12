use super::MasterOfCeremonyHandler;
use super::StepsHandler;
use crate::cli::Opt;

use std::collections::HashMap;

/// Filters for clie events.
#[derive(Debug, Default, Clone)]
pub struct CliFiltersHandler {
    pub cli: Opt,
}

impl CliFiltersHandler {
    pub fn new() -> CliFiltersHandler {
        CliFiltersHandler { cli: Opt::new() }
    }

    pub fn process(&mut self, model: &mut MasterOfCeremonyHandler) {
        self.filter_env(model);
        self.filter_repo(model);
        self.filter_flow(model);
        self.filter_steps(model);
    }

    pub fn filter_env(&mut self, model: &mut MasterOfCeremonyHandler) {
        match &self.cli.env {
            Some(x) => {
                model.data.specs.set_env_file_path(x.to_path_buf());
            }
            None => {}
        }
    }

    pub fn filter_repo(&mut self, model: &mut MasterOfCeremonyHandler) {
        match &self.cli.repo {
            Some(x) => {
                model.data.specs.set_repo_url(x.to_string());
            }
            None => {}
        }
    }

    pub fn filter_flow(&mut self, model: &mut MasterOfCeremonyHandler) {
        match &self.cli.flow {
            Some(flow) => {
                model.data.specs.set_flow(flow);
            }
            None => {
                // If no flow is defined then set to None so default flow is applied.
                // (ie: All available steps.)
                model.data.specs.flows = None;
            }
        }
    }

    pub fn filter_steps(&mut self, model: &mut MasterOfCeremonyHandler) {
        // Filter option setting for script, template, template_out, params and post_script).
        let mut set_steps = false;
        let mut handler = StepsHandler::new();

        if self.cli.script.is_some() {
            set_steps = true;
        }
        if self.cli.template.is_some() {
            set_steps = true;
        }
        if self.cli.template_out.is_some() {
            set_steps = true;
        }
        if self.cli.param.is_some() {
            set_steps = true;
        }
        if self.cli.post_script.is_some() {
            set_steps = true;
        }

        if set_steps {
            model.prompt = false;
            model.data.specs.flows = None;
            model.data.specs.steps = HashMap::new();
            model.data.specs.steps.insert(
                "pre".to_string(),
                handler.create_step(
                    self.cli.script.to_owned(),
                    self.cli.template.to_owned(),
                    self.cli.template_out.to_owned(),
                    self.cli.param.to_owned(),
                    self.cli.post_script.to_owned(),
                ),
            );
        }
    }
}
