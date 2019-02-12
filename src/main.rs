extern crate clap;
extern crate yaml_rust;
extern crate handlebars;
extern crate serde_json;
extern crate run_script;
extern crate dialoguer;
extern crate dotenv;
extern crate console;
extern crate git2;

mod cli;
mod model;
mod handler;
mod file;

use crate::model::{Scripts};
use crate::handler::{Configs, Steps, Repo};
use crate::file::{EnvFile};

fn main() {
    let matches = cli::build_cli().get_matches();
    let mute = matches.is_present("mute");
    
    let request = Configs::process_args(matches.to_owned());
    let mut scripts = Scripts::new();
    
    scripts.load_scripts(&request); 
    
    let no_prompt = request.has_no_prompt();
    
    let mut flow = (
        "default".to_string(), 
        "".to_string(), 
        vec![
             "pre-script".to_string(), 
             "unit-test".to_string(), 
             "build-script".to_string(), 
             "functional-test".to_string(), 
             "template".to_string(), 
             "deploy-script".to_string(), 
             "system-test".to_string(), 
             "post-script".to_string()
            ]
    );
    
    // Clone repository if config entry exist.
    Repo::check_and_process(&request);
    
    EnvFile::check_and_load(&matches);
    if request.has_flow() {
        flow = request.flow().to_owned();
        if flow.1.len() < 1 {
            EnvFile::load_env(flow.1);
        }
    }
    
    EnvFile::run_env_prompt(&request.global_env(), &no_prompt);
    for x in flow.2 {
        if x == "pre-script".to_string() {
            Steps::run_pre(&request, &scripts, no_prompt, mute);
        } else if x == "unit-test".to_string() {
            Steps::run_unit_test(&request, &scripts, no_prompt, mute);
        } else if x == "build-script".to_string() {
            Steps::run_build(&request, &scripts, no_prompt, mute);
        } else if x == "functional-test".to_string() {
            Steps::run_functional_test(&request, &scripts, no_prompt, mute);
        } else if x == "template".to_string() {
            Steps::run_template(&request, &scripts, no_prompt, mute);
        } else if x == "deploy-script".to_string() {
            Steps::run_deploy(&request, &scripts, no_prompt, mute);
        } else if x == "system-test".to_string() {
            Steps::run_system_test(&request, &scripts, no_prompt, mute);
        } else if x == "post-script".to_string() {
            Steps::run_post(&request, &scripts, no_prompt, mute);
        }
    }
    
    println!("\nDone!");
}