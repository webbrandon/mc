use file::file_to_string;
use std::fmt;
use model::{Configs};

pub struct Scripts {
    build: String,
    param: String,
    post: String,
    deploy: String
}

impl Scripts {
    pub fn new() -> Scripts {
        Scripts {
            build: String::new(),
            param: String::new(),
            post: String::new(),
            deploy: String::new()
        }
    }
    pub fn set_build_script(&mut self, build: String) {
        self.build = build;
    }
    pub fn set_param_script(&mut self, param: String) {
        self.param = param;
    }
    pub fn set_deploy_script(&mut self, deploy: String) {
        self.deploy = deploy;
    }
    pub fn set_post_script(&mut self, post: String) {
        self.post = post;
    }
    pub fn build_script(&self) -> &String {
        &self.build
    }
    pub fn param_script(&self) -> &String {
        &self.param
    }
    pub fn post_script(&self) -> &String {
        &self.post
    }
    pub fn deploy_script(&self) -> &String {
        &self.deploy
    }
    pub fn load_scripts(&mut self, request: &Configs) -> &Scripts {
        if request.has_build_script() {
            self.set_build_script(file_to_string(request.build_script()));
        }
        if request.has_param_script() {
            self.set_param_script(file_to_string(request.param_script()));
        }
        if request.has_deploy_script() {
            self.set_deploy_script(file_to_string(request.deploy_script()));
        }
        if request.has_post_script() {
            self.set_post_script(file_to_string(request.post_script()));
        }
        self
    }
}

impl fmt::Debug for Scripts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scripts {{\n\t build: \n{}\n\t post: \n{}\n\t deploy: \n{}\n}}",
        self.build, self.post, self.deploy)
    }
}

