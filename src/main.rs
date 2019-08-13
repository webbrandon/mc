#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate clap;
extern crate handlebars;
extern crate run_script;
extern crate serde_json;
extern crate serde_yaml;
extern crate structopt;

mod cli;
mod completions;
mod handlers;
mod models;

use cli::{Commands, Opt};
use completions::CompletionProcess;
use structopt::StructOpt;

fn main() {
    let mut mc_api = handlers::MasterOfCeremonyHandler::new();

    // Resolve any commandline request that do not process api's.
    let cli_settings: Opt = cli::Opt::from_args();

    match cli_settings.commands {
        Some(commands) => match commands {
            Commands::Completions(x) => {
                let ran_completion = CompletionProcess::run(x);
                if ran_completion {
                    std::process::exit(0);
                }
            }
        },
        None => {
            // Load the yaml config if it exist.  This will allow users to choose api specifications.
            // Advanced feature sets are available in configuration files.
            debug!("Loading yaml config");
            
            mc_api = handlers::MasterOfCeremonyHandler::new();
            mc_api.add_cli(cli_settings.clone());
            mc_api.load_file(&cli_settings.file);
        }
    }
    mc_api.process();
    println!("\nDone!");
}
