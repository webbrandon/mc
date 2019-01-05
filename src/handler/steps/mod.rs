use dialoguer::{Confirmation, theme::ColorfulTheme};
use crate::model::{Scripts, Template};
use crate::handler::{Configs};
use crate::file;
use crate::file::{EnvFile};

fn log_it(mute: bool, content: String) {
    if ! mute {
        file::out_term(&content);
    }
}

pub fn continue_prompt(step_name: String, ask: &bool) -> bool {
    let mut is_requested = true;
    if !ask.to_owned() {
        let question = format!("Do you want to continue {} step?", step_name);
        if Confirmation::with_theme(&ColorfulTheme::default())
            .with_text(question.as_str())
            .interact()
            .unwrap()
        {
            println!("");
        } else {
            println!("Skipping {} step.", step_name);
            is_requested = false;
        }
    }
    is_requested
}

pub struct Steps {}

impl Steps {
    pub fn run_build(request: &Configs, scripts: &Scripts, no_prompt: bool, mute: bool) {
        if request.has_build_script() {
            if !request.has_no_build() {
                let ask_question = continue_prompt("build script".to_string(), &no_prompt);
                if ask_question {
                    EnvFile::run_env_prompt(&request.build_env(), &no_prompt);
                    log_it(mute, scripts.process_build_script().to_owned());
                }
            }
        }
    }

    pub fn run_template(request: &Configs, scripts: &Scripts, no_prompt: bool, mute: bool) {    
        if request.has_template() && request.has_params() {
            if !request.has_no_template() {
                let ask_question = continue_prompt("and apply parameters to template".to_string(), &no_prompt);
                if ask_question {
                    EnvFile::run_env_prompt(&request.param_env(), &no_prompt);
                    if request.has_param_script() {
                        log_it(mute, scripts.process_param_script().to_owned());
                    }
                    let mut templates = Template::new();
                    templates.load_templates(request);
                    templates.render_template();
                    if request.has_render() {
                        file::outfile(request.render(), templates.render());
                        log_it(mute, templates.render().to_string());
                    }
                }
            }
        }
    }
    
    pub fn run_deploy(request: &Configs, scripts: &Scripts, no_prompt: bool, mute: bool) { 
        if request.has_deploy_script() {
            if !request.has_no_deploy() {
                let ask_question = continue_prompt("deploy script".to_string(), &no_prompt);
                if ask_question {
                    EnvFile::run_env_prompt(&request.deploy_env(), &no_prompt);
                    log_it(mute, scripts.process_deploy_script().to_owned());
                }
            }
        }
    }
    
    pub fn run_post(request: &Configs, scripts: &Scripts, no_prompt: bool, mute: bool) { 
        if request.has_post_script() {
            if !request.has_no_post() {
                let ask_question = continue_prompt("post script".to_string(), &no_prompt);
                if ask_question {
                    EnvFile::run_env_prompt(&request.post_env(), &no_prompt);
                    log_it(mute, scripts.process_post_script());        
                }
            }
        }
    }
}
