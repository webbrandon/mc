extern crate clap;
extern crate yaml_rust;
extern crate handlebars;
extern crate serde_json;
extern crate run_script;
extern crate dialoguer;
extern crate dotenv;

mod cli;
mod model;
mod handler;
mod file;

use dialoguer::{Confirmation, theme::ColorfulTheme};
use model::{Scripts, Template};
use handler::{Configs};
use file::{EnvFile};

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
            println!("Now running...");
        } else {
            println!("Skipping {} step.", step_name);
            is_requested = false;
        }
    }
    is_requested
}

fn main() {
    let matches = cli::build_cli().get_matches();
    let mut request = Configs::process_args(&matches);
    let mut scripts = Scripts::new();
    let mute = matches.is_present("mute");
    
    EnvFile::check_and_load(&matches);
    scripts.load_scripts(&mut request); 
    
    let no_prompt = request.has_no_prompt();
    
    if !request.has_no_build() {
        let ask_question = continue_prompt("build script".to_string(), &no_prompt);
        if ask_question {
            if request.has_build_script() {
                EnvFile::run_env_prompt(&request.build_env(), &no_prompt);
                log_it(mute, scripts.process_build_script().to_owned());
            }
        }
    }
    
    if !request.has_no_template() {
        let ask_question = continue_prompt("and apply parameters to template".to_string(), &no_prompt);
        if ask_question {
            if request.has_template() && request.has_params() {
                EnvFile::run_env_prompt(&request.param_env(), &no_prompt);
                if request.has_param_script() {
                    // Need to dig deep I did something that took ownership so I reinitialize with Configs.
                    scripts = Scripts::new();
                    scripts.load_scripts(&mut request); 
                    log_it(mute, scripts.process_param_script().to_owned());
                }
                let mut templates = Template::new();
                templates.load_templates(&mut request);
                templates.render_template();
                if request.has_render() {
                    file::outfile(request.render(), templates.render());
                    log_it(mute, templates.render().to_string());
                }
            }
        }
    }
    
    if !request.has_no_deploy() {
        let ask_question = continue_prompt("deploy script".to_string(), &no_prompt);
        if ask_question {
            if request.has_deploy_script() {
                // Need to dig deep I did something that took ownership so I reinitialize with Configs.
                scripts = Scripts::new();
                scripts.load_scripts(&mut request); 
                EnvFile::run_env_prompt(&request.deploy_env(), &no_prompt);
                log_it(mute, scripts.process_deploy_script().to_owned());
            }
        }
    }
    
    if !request.has_no_post() {
        let ask_question = continue_prompt("post script".to_string(), &no_prompt);
        if ask_question {
            if request.has_post_script() {
                // Need to dig deep I did something that took ownership so I reinitialize with Configs.
                scripts = Scripts::new();
                scripts.load_scripts(&mut request); 
                EnvFile::run_env_prompt(&request.post_env(), &no_prompt);
                log_it(mute, scripts.process_post_script());        
            }
        }
    }
}