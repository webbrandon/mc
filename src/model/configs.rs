use std::fmt;

pub struct Configs {
    params: String,
    template: String,
    render: String,
    script: String,
    post_script: String
}

impl Configs {
    pub fn new() -> Configs {
        Configs {
            params: String::new(),
            template: String::new(),
            render: String::new(),
            script: String::new(),
            post_script: String::new()
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
    pub fn set_script(&mut self, script: String) {
        self.script = script;
    }
    pub fn set_post_script(&mut self, post_script: String) {
        self.post_script = post_script;
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
    pub fn script(&mut self) -> &String {
        &self.script
    }
    pub fn post_script(&mut self) -> &String {
        &self.post_script
    }
    pub fn has_params(&mut self) -> bool {
        match self.params.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
    pub fn has_template(&mut self) -> bool {
        match self.template.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
    pub fn has_render(&mut self) -> bool {
        match self.render.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
    pub fn has_script(&mut self) -> bool {
        match self.script.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
    pub fn has_post_script(&mut self) -> bool {
        match self.post_script.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
}

impl fmt::Debug for Configs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Configs {{\n\t template: {}\n\t params: {}\n\t script: {}\n\t post-script: {}\n}}",
        self.template, self.params, self.script, self.post_script)
    }
}