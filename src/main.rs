extern crate clap;
extern crate yaml_rust;
extern crate handlebars;
extern crate serde_json;
extern crate run_script;

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

fn main() {
    let matches = cli::build_cli().get_matches();
    let request = &mut Configs::process_args(&matches);
    let mut scripts = Scripts::new();
    let mut templates = Template::new();
    let mute = matches.is_present("mute");
    
    templates.load_templates(request);
    scripts.load_scripts(request);
    
    if request.has_script() {
        log_it(mute, scripts.process_script());
    }
    
    if request.has_template() && request.has_params() {
        templates.render_template();
        if request.has_render() {
            file::outfile(request.render(), templates.render());
            log_it(mute, templates.render().to_string());
        }
    }
    
    if request.has_post_script() {
        log_it(mute, scripts.process_post_script());
    }
}