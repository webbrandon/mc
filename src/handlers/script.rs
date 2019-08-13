use super::file;

use run_script;
use run_script::ScriptOptions;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process;

/// The ScriptHandler will process a script an manage the outpout.
#[derive(Debug, Default)]
pub struct ScriptHandler {
    pub script_path: PathBuf,
    pub mute: bool,
}

impl ScriptHandler {
    pub fn new() -> ScriptHandler {
        Default::default()
    }

    pub fn process(&mut self) -> bool {
        let (code, _output, error) = run_script::run(
            &file::load(self.script_path.to_owned()),
            &vec![],
            &self.get_options(),
        )
        .unwrap();

        if code == 0 {
            true
        } else {
            writeln!(
                &mut io::stderr(),
                "!! format promblem !! {} : {}",
                code,
                error
            )
            .ok();
            process::exit(1)
        }
    }

    pub fn get_options(&mut self) -> ScriptOptions {
        let mut options = ScriptOptions::new();

        options.runner = None;
        options.exit_on_error = true;
        if self.mute {
            options.capture_output = true;
            options.print_commands = false;
        } else if !self.mute {
            options.capture_output = false;
            options.print_commands = true;
        }

        options
    }
}
