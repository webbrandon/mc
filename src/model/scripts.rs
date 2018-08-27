use file::file_to_string;
use std::fmt;
use model::{Configs};

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
    pub fn script(&mut self) -> &String {
        &self.pre
    }
    pub fn post_script(&mut self) -> &String {
        &self.post
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
}

impl fmt::Debug for Scripts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scripts {{\n\t pre: \n{}\n\t post: \n{}\n}}",
        self.pre, self.post)
    }
}

