use file::file_to_string;
use std::fmt;
use run_script;
use run_script::ScriptOptions;
use file::bad_format_close_app;
use model::{Configs};

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

pub struct Scripts {
    pre: String,
    post: String
}

impl Scripts {
    pub fn new() -> Scripts {
        Scripts {
            pre: String::new(),
            post: String::new()
        }
    }
    pub fn set_script(&mut self, pre: String) {
        self.pre = pre;
    }
    pub fn set_post_script(&mut self, post: String) {
        self.post = post;
    }
    pub fn load_scripts(&mut self, request: &mut Configs) -> &Scripts {
        if request.has_script() {
            self.set_script(file_to_string(request.script()));
        }
        if request.has_post_script() {
            self.set_post_script(file_to_string(request.post_script()));
        }
        self
    }
    pub fn process_script(&mut self) -> String {
        run_build_script(&self.pre)
    }
    pub fn process_post_script(&mut self) -> String {
        run_build_script(&self.post)
    }
}

impl fmt::Debug for Scripts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scripts {{\n\t pre: \n{}\n\t post: \n{}\n}}",
        self.pre, self.post)
    }
}

