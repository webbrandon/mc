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

use crate::model::{Scripts};
use crate::handler::{Configs, Steps};
use crate::file::{EnvFile};


fn main() {
    let matches = cli::build_cli().get_matches();
    let mute = matches.is_present("mute");
    EnvFile::check_and_load(&matches);
    
    let request = Configs::process_args(matches);
    let mut scripts = Scripts::new();
    
    scripts.load_scripts(&request); 
    
    let no_prompt = request.has_no_prompt();
    
    Steps::run_build(&request, &scripts, no_prompt, mute);
    Steps::run_template(&request, &scripts, no_prompt, mute);
    Steps::run_deploy(&request, &scripts, no_prompt, mute);
    Steps::run_post(&request, &scripts, no_prompt, mute);
    
    println!("\nDone!");
}