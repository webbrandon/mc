use super::env_prompt;
use super::script;
use super::template;

use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirmation;

use crate::models::steps::Step;
use crate::models::template::Template;
use std::path::PathBuf;

/// The StepsHandler will process Step request.
#[derive(Debug, Default, Clone)]
pub struct StepsHandler {}

impl StepsHandler {
    pub fn new() -> StepsHandler {
        Default::default()
    }

    pub fn process(&mut self, step_model: Vec<(String, Step)>, prompt: bool, mute: bool) {
        for step in step_model {
            let mut ran_step = false;
            if prompt {
                if self.continue_prompt(step.0) {
                    ran_step = self.process_step(step.1, prompt, mute);
                } else {ran_step = true;}
            } else if !prompt {
                ran_step = self.process_step(step.1, prompt, mute);
            } 
            
            if !ran_step {
                eprintln!("Step Failed!");
                std::process::exit(1);
            }
        }
    }

    pub fn process_step(&mut self, step: Step, prompt: bool, mute: bool) -> bool {
        let mut ran_step = false;

        if prompt {
            match step.clone().env_prompt {
                Some(x) => {
                    env_prompt::EnvironmentPromptHandler::process(&x);
                }
                None => {}
            }
        }

        match step.clone().script {
            Some(x) => {
                ran_step = self.process_script(x, mute);
            }
            None => {}
        }

        if step.templates.is_some() {
            ran_step = self.process_template(step.clone(), mute);
        }

        match step.clone().post_script {
            Some(x) => {
                ran_step = self.process_script(x, mute);
            }
            None => {}
        }

        ran_step
    }

    pub fn process_script(&mut self, script_path: PathBuf, mute: bool) -> bool {
        let mut handler = script::ScriptHandler::new();
        handler.mute = mute;
        handler.script_path = script_path;

        handler.process()
    }

    pub fn process_template(&mut self, step_model: Step, mute: bool) -> bool {
        let mut handler = template::TemplateHandler::new();
        handler.mute = mute;

        match step_model.templates {
            Some(x) => {
                handler.template = x;
            }
            None => {}
        }

        handler.process()
    }

    pub fn create_step(
        &mut self,
        script: Option<PathBuf>,
        template: Option<PathBuf>,
        template_out: Option<PathBuf>,
        param: Option<PathBuf>,
        post_script: Option<PathBuf>,
    ) -> Step {
        let mut step = Step::new_empty();
        let mut template_model = Template::new();
        template_model.template = template;
        template_model.out_file = template_out;
        template_model.params = param;
        
        step.script = script;
        step.templates = Some(vec![template_model]);
        step.post_script = post_script;

        step
    }

    pub fn continue_prompt(&mut self, step_name: String) -> bool {
        let question = format!("Do you want to continue {} step?", step_name);
        if Confirmation::with_theme(&ColorfulTheme::default())
            .with_text(question.as_str())
            .interact()
            .unwrap()
        {
            true
        } else {
            println!("Skipping {} step.", step_name);
            false
        }
    }
}
