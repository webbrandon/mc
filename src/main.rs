extern crate clap;
extern crate yaml_rust;
extern crate handlebars;
extern crate serde_json;
extern crate run_script;
extern crate dialoguer;
extern crate dotenv;
extern crate console;

mod cli;
mod model;
mod handler;
mod file;

use crate::model::{Scripts};
use crate::handler::{Configs, Steps};
use crate::file::{EnvFile};

fn main() {
    let matches = cli::build_cli().get_matches();
    let mute = matches.is_present("mute");
    EnvFile::check_and_load(&matches);
    
    let request = Configs::process_args(matches.to_owned());
    let mut scripts = Scripts::new();
    
    scripts.load_scripts(&request); 
    
    let no_prompt = request.has_no_prompt();
    
    let mut flow = ("default".to_string(), "".to_string(), vec!["build-script".to_string(), "template".to_string(), "deploy-script".to_string(), "post-script".to_string()]);
    if request.has_flow() {
        flow = request.flow().to_owned();
        if flow.1.len() < 1 {
            EnvFile::load_env(flow.1);
        }
    }
    
    EnvFile::run_env_prompt(&request.global_env(), &no_prompt);
    for x in flow.2 {
        if x == "build-script".to_string() {
            Steps::run_build(&request, &scripts, no_prompt, mute);
        } else if x == "template".to_string() {
            Steps::run_template(&request, &scripts, no_prompt, mute);
        } else if x == "deploy-script".to_string() {
            Steps::run_deploy(&request, &scripts, no_prompt, mute);
        } else if x == "post-script".to_string() {
            Steps::run_post(&request, &scripts, no_prompt, mute);
        }
    }
    
    println!("\nDone!");
}