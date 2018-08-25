use std::fmt;
use file::parse_json;
use std::process;
use handlebars::Handlebars;
use file::file_to_string;
use model::{Configs};

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
    pub fn render(&mut self) -> &String {
        &self.render
    }
    pub fn load_templates(&mut self, request: &mut Configs) -> &Template {
        if request.has_params() && request.has_template() {
            self.set_params(file_to_string(request.params()));
            self.set_template(file_to_string(request.template()));
        }
        self
    }
    pub fn render_template(&mut self) -> &String {
        let json = parse_json(&self.params);
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("template", &self.template)
            .ok()
            .unwrap();
        match handlebars.render("template", &json) {
            Ok(data) => {
                self.set_render(data);
                &self.render
            }
            Err(_e) => {
                process::exit(2);
            }
        }
    }
}

impl fmt::Debug for Template {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Template {{\n\t template: \n{}\n\t params: \n{}\n}}",
        self.template, self.params)
    }
}
