use run_script;
use run_script::ScriptOptions;
use file::bad_format_close_app;
pub use model::{Scripts};

pub fn run_build_script(script: &String) -> String {
    let mut options = ScriptOptions::new();
    options.runner = None; 
    options.capture_output = true; 
    options.exit_on_error = false; 
    options.print_commands = true;

    let args = vec![];

    let (code, output, error) = run_script::run(
        script,
        &args,
        &options,
    ).unwrap();
    if code == 0 {
        output
    } else {
        println!("{} : {}", code, error);
        bad_format_close_app();
    }
}

impl Scripts {
    pub fn process_script(mut self) -> String {
        let script = &self.script();
        run_build_script(&script)
    }
    pub fn process_post_script(mut self) -> String {
        let script = &self.post_script();
        run_build_script(&script)
    }
}








