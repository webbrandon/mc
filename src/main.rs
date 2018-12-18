extern crate clap;
extern crate yaml_rust;
extern crate handlebars;
extern crate serde_json;
extern crate run_script;

use std::io;

mod cli;
mod model;
mod handler;
mod file;

use model::{Scripts, Template};
use handler::{Configs};

fn log_it(mute: bool, content: String) {
    if ! mute {
        file::out_term(&content);
    }
}

fn continue_prompt(step_name: String, ask: bool) {
    if ask {
        println!("Would you like to continue with the {} step? (y/N)", step_name);
        let buffer = &mut String::new();
        io::stdin().read_line(buffer);
        match buffer.trim_right() {
            "true" => true,
            "TRUE" => true,
            "True" => true,
            "T" => true,
            "t" => true,
            "false" => false,
            "FALSE" => false,
            "False" => false,
            "F" => false,
            "f" => false,
            "" => false,
            _ => false
        };
    }
}

fn main() {
    let matches = cli::build_cli().get_matches();
    let mut request = Configs::process_args(&matches);
    let mut scripts = Scripts::new();
    let mute = matches.is_present("mute");
    scripts.load_scripts(&mut request); 
    
    if !request.has_no_build() {
        if request.has_build_script() {
            log_it(mute, scripts.process_build_script().to_owned());
        }
    }
    
    if !request.has_no_template() {
        continue_prompt("template".to_string(), request.has_no_prompt());
        if request.has_template() && request.has_params() {
            let mut templates = Template::new();
            templates.load_templates(&mut request);
            templates.render_template();
            if request.has_param_script() {
                // Need to dig deep I did something that took ownership so I reinitialize with Configs.
                scripts = Scripts::new();
                scripts.load_scripts(&mut request); 
                log_it(mute, scripts.process_param_script().to_owned());
            }
            if request.has_render() {
                file::outfile(request.render(), templates.render());
                log_it(mute, templates.render().to_string());
            }
        }
    }
    
    if !request.has_no_deploy() {
        continue_prompt("deploy script".to_string(), request.has_no_prompt());
        if request.has_deploy_script() {
            // Need to dig deep I did something that took ownership so I reinitialize with Configs.
            scripts = Scripts::new();
            scripts.load_scripts(&mut request); 
            log_it(mute, scripts.process_deploy_script().to_owned());
        }
    }
    
    if !request.has_no_post() {
        continue_prompt("post script".to_string(), request.has_no_prompt());
        if request.has_post_script() {
            // Need to dig deep I did something that took ownership so I reinitialize with Configs.
            scripts = Scripts::new();
            scripts.load_scripts(&mut request); 
            log_it(mute, scripts.process_post_script());        
        }
    }
}