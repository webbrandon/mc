use std::fmt;
use crate::file::file_to_string;
use crate::model::{Configs};

pub struct Template {
    params: String,
    template: String,
    render: String
}

impl Template {
    pub fn new() -> Template {
        Template {
            params: String::new(),
            template: String::new(),
            render: String::new()
        }
    }
    pub fn set_params(&mut self, params: String) {
        self.params = params;
    }
    pub fn set_template(&mut self, template: String) {
        self.template = template;
    }
    pub fn set_render(&mut self, render: String) {
        self.render = render;
    }
    pub fn params(&mut self) -> &String {
        &self.params
    }
    pub fn template(&mut self) -> &String {
        &self.template
    }
    pub fn render(&mut self) -> &String {
        &self.render
    }
    pub fn load_templates(&mut self, request: &Configs) -> &Template {
        if request.has_params() && request.has_template() {
            let remote_check = &request.params()[..4].to_string();
            if remote_check.contains("http") {
                let json = "{\"name\": \"Brandon\"}".to_string();
                self.set_params(json);
            } else {
                self.set_params(file_to_string(request.params()));
            }
            self.set_template(file_to_string(request.template()));
        }
        self
    }
}

impl fmt::Debug for Template {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Template {{\n\t template: \n{}\n\t params: \n{}\n}}",
        self.template, self.params)
    }
}
