use std::fmt;

pub struct Configs {
    params: String,
    template: String,
    render: String,
    build_script: String,
    param_script: String,
    deploy_script: String,
    post_script: String,
    no_build: String,
    no_deploy: String,
    no_template: String,
    no_post: String,
    no_prompt: String
}

impl Configs {
    pub fn new() -> Configs {
        Configs {
            params: String::new(),
            template: String::new(),
            render: String::new(),
            build_script: String::new(),
            deploy_script: String::new(),
            param_script: String::new(),
            post_script: String::new(),
            no_build: String::new(),
            no_deploy: String::new(),
            no_template: String::new(),
            no_post: String::new(),
            no_prompt: String::new()
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
    pub fn set_build_script(&mut self, script: String) {
        self.build_script = script;
    }
    pub fn set_param_script(&mut self, script: String) {
        self.param_script = script;
    }
    pub fn set_deploy_script(&mut self, script: String) {
        self.deploy_script = script;
    }
    pub fn set_post_script(&mut self, post_script: String) {
        self.post_script = post_script;
    }
    pub fn set_no_build(&mut self, setting: String) {
        self.no_build = setting;
    }
    pub fn set_no_deploy(&mut self, setting: String) {
        self.no_deploy = setting;
    }
    pub fn set_no_template(&mut self, setting: String) {
        self.no_template = setting;
    }
    pub fn set_no_post(&mut self, setting: String) {
        self.no_post = setting;
    }
    pub fn set_no_prompt(&mut self, setting: String) {
        self.no_prompt = setting;
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
    pub fn build_script(&mut self) -> &String {
        &self.build_script
    }
    pub fn param_script(&mut self) -> &String {
        &self.param_script
    }
    pub fn deploy_script(&mut self) -> &String {
        &self.deploy_script
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
    pub fn has_build_script(&mut self) -> bool {
        match self.build_script.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
    pub fn has_param_script(&mut self) -> bool {
        match self.param_script.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
    pub fn has_deploy_script(&mut self) -> bool {
        match self.deploy_script.len()  {
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
    pub fn has_no_build(&mut self) -> bool {
        match self.no_build.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
    pub fn has_no_deploy(&mut self) -> bool {
        match self.no_deploy.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
    pub fn has_no_template(&mut self) -> bool {
        match self.no_template.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
    pub fn has_no_post(&mut self) -> bool {
        match self.no_post.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
    pub fn has_no_prompt(&mut self) -> bool {
        match self.no_prompt.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
}

impl fmt::Debug for Configs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Configs {{\n\t template: {}\n\t params: {}\n\t params-script: {}\n\t build-script: {}\n\t deploy-script: {}\n\t post-script: {}\n}}",
        self.template, self.params, self.param_script, self.build_script, self.deploy_script, self.post_script)
    }
}