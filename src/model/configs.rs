use std::fmt;

#[derive(Default)]
pub struct Configs {
    params: String,
    template: String,
    render: String,
    global_env: Vec<(String, String, String, String, Vec<String>)>,
    build_script: String,
    build_env: Vec<(String, String, String, String, Vec<String>)>,
    param_script: String,
    param_env: Vec<(String, String, String, String, Vec<String>)>,
    deploy_script: String,
    deploy_env: Vec<(String, String, String, String, Vec<String>)>,
    post_script: String,
    post_env: Vec<(String, String, String, String, Vec<String>)>,
    no_build: bool,
    no_deploy: bool,
    no_template: bool,
    no_post: bool,
    no_prompt: bool,
    flow_name: String,
    flow: (String, String, Vec<String>)
}

impl Configs {
    pub fn new() -> Configs {
        Default::default()
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
    pub fn set_global_env(&mut self, env: Vec<(String, String, String, String, Vec<String>)>) {
        self.global_env = env;
    }
    pub fn set_build_script(&mut self, script: String) {
        self.build_script = script;
    }
    pub fn set_build_env(&mut self, env_options: Vec<(String, String, String, String, Vec<String>)>) {
        self.build_env = env_options;
    }
    pub fn set_param_script(&mut self, script: String) {
        self.param_script = script;
    }
    pub fn set_param_env(&mut self, env_options: Vec<(String, String, String, String, Vec<String>)>) {
        self.param_env = env_options;
    }
    pub fn set_deploy_script(&mut self, script: String) {
        self.deploy_script = script;
    }
    pub fn set_deploy_env(&mut self, env_options: Vec<(String, String, String, String, Vec<String>)>) {
        self.deploy_env = env_options;
    }
    pub fn set_post_script(&mut self, post_script: String) {
        self.post_script = post_script;
    }
    pub fn set_post_env(&mut self, env_options: Vec<(String, String, String, String, Vec<String>)>) {
        self.post_env = env_options;
    }
    pub fn set_no_build(&mut self, setting: bool) {
        self.no_build = setting;
    }
    pub fn set_no_deploy(&mut self, setting: bool) {
        self.no_deploy = setting;
    }
    pub fn set_no_template(&mut self, setting: bool) {
        self.no_template = setting;
    }
    pub fn set_no_post(&mut self, setting: bool) {
        self.no_post = setting;
    }
    pub fn set_no_prompt(&mut self, setting: bool) {
        self.no_prompt = setting;
    }
    pub fn set_flow_name(&mut self, flow_name: String) {
        self.flow_name = flow_name;
    }
    pub fn set_flow(&mut self, flow: (String, String, Vec<String>)) {
        self.flow = flow;
    }
    pub fn params(&self) -> &String {
        &self.params
    }
    pub fn template(&self) -> &String {
        &self.template
    }
    pub fn render(&self) -> &String {
        &self.render
    }
    pub fn build_script(&self) -> &String {
        &self.build_script
    }
    pub fn global_env(&self) -> &Vec<(String, String, String, String, Vec<String>)> {
        &self.global_env
    }
    pub fn build_env(&self) -> &Vec<(String, String, String, String, Vec<String>)> {
        &self.build_env
    }
    pub fn param_script(&self) -> &String {
        &self.param_script
    }
    pub fn param_env(&self) -> &Vec<(String, String, String, String, Vec<String>)> {
        &self.param_env
    }
    pub fn deploy_script(&self) -> &String {
        &self.deploy_script
    }
    pub fn deploy_env(&self) -> &Vec<(String, String, String, String, Vec<String>)> {
        &self.deploy_env
    }
    pub fn post_script(&self) -> &String {
        &self.post_script
    }
    pub fn post_env(&self) -> &Vec<(String, String, String, String, Vec<String>)> {
        &self.post_env
    }
    pub fn flow_name(&self) -> &String {
        &self.flow_name
    }
    pub fn flow(&self) -> &(String, String, Vec<String>) {
        &self.flow
    }
    pub fn has_params(&self) -> bool {
        match self.params.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
    pub fn has_template(&self) -> bool {
        match self.template.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
    pub fn has_render(&self) -> bool {
        match self.render.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
    pub fn has_build_script(&self) -> bool {
        match self.build_script.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
    pub fn has_param_script(&self) -> bool {
        match self.param_script.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
    pub fn has_deploy_script(&self) -> bool {
        match self.deploy_script.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
    pub fn has_post_script(&self) -> bool {
        match self.post_script.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
    pub fn has_no_build(&self) -> bool {
        self.no_build
    }
    pub fn has_no_deploy(&self) -> bool {
        self.no_deploy
    }
    pub fn has_no_template(&self) -> bool {
        self.no_template
    }
    pub fn has_no_post(&self) -> bool {
        self.no_post
    }
    pub fn has_no_prompt(&self) -> bool {
        self.no_prompt
    }
    pub fn has_flow(&self) -> bool {
        match self.flow_name.len()  {
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