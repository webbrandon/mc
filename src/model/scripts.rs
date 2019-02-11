use crate::file::file_to_string;
use std::fmt;
use crate::model::{Configs};

pub struct Scripts {
    pre: String,
    unit: String,
    build: String,
    functional: String,
    param: String,
    post: String,
    deploy: String,
    system: String
}

impl Scripts {
    pub fn new() -> Scripts {
        Scripts {
            pre: String::new(),
            unit: String::new(),
            build: String::new(),
            functional: String::new(),
            param: String::new(),
            post: String::new(),
            deploy: String::new(),
            system: String::new()
        }
    }
    pub fn set_pre_script(&mut self, build: String) {
        self.build = build;
    }
    pub fn set_unit_test(&mut self, unit: String) {
        self.unit = unit;
    }
    pub fn set_build_script(&mut self, build: String) {
        self.build = build;
    }
    pub fn set_functional_test(&mut self, functional: String) {
        self.functional = functional;
    }
    pub fn set_param_script(&mut self, param: String) {
        self.param = param;
    }
    pub fn set_deploy_script(&mut self, deploy: String) {
        self.deploy = deploy;
    }
    pub fn set_system_test(&mut self, system: String) {
        self.system = system;
    }
    pub fn set_post_script(&mut self, post: String) {
        self.post = post;
    }
    pub fn pre_script(&self) -> &String {
        &self.build
    }
    pub fn unit_test(&self) -> &String {
        &self.unit
    }
    pub fn build_script(&self) -> &String {
        &self.build
    }
    pub fn functional_test(&self) -> &String {
        &self.functional
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
    pub fn system_test(&self) -> &String {
        &self.system
    }
    pub fn load_scripts(&mut self, request: &Configs) -> &Scripts {
        if request.has_pre_script() {
            self.set_pre_script(file_to_string(request.pre_script()));
        }
        if request.has_unit_test() {
            self.set_unit_test(file_to_string(request.unit_test()));
        }
        if request.has_build_script() {
            self.set_build_script(file_to_string(request.build_script()));
        }
        if request.has_functional_test() {
            self.set_functional_test(file_to_string(request.functional_test()));
        }
        if request.has_param_script() {
            self.set_param_script(file_to_string(request.param_script()));
        }
        if request.has_deploy_script() {
            self.set_deploy_script(file_to_string(request.deploy_script()));
        }
        if request.has_system_test() {
            self.set_system_test(file_to_string(request.system_test()));
        }
        if request.has_post_script() {
            self.set_post_script(file_to_string(request.post_script()));
        }
        self
    }
}

impl fmt::Debug for Scripts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scripts {{\n\t pre: \n{}\n\t build: \n{}\n\t post: \n{}\n\t deploy: \n{}\n}}", self.pre,
        self.build, self.post, self.deploy)
    }
}

