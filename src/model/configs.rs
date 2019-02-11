use std::fmt;

#[derive(Default)]
pub struct Configs {
    params: String,
    template: String,
    render: String,
    global_env: Vec<(String, String, String, String, Vec<String>)>,
    pre_script: String,
    pre_env: Vec<(String, String, String, String, Vec<String>)>,
    unit_test: String,
    unit_test_env: Vec<(String, String, String, String, Vec<String>)>,
    build_script: String,
    build_env: Vec<(String, String, String, String, Vec<String>)>,
    functional_test: String,
    functional_test_env: Vec<(String, String, String, String, Vec<String>)>,
    param_script: String,
    param_env: Vec<(String, String, String, String, Vec<String>)>,
    deploy_script: String,
    deploy_env: Vec<(String, String, String, String, Vec<String>)>,
    system_test: String,
    system_test_env: Vec<(String, String, String, String, Vec<String>)>,
    post_script: String,
    post_env: Vec<(String, String, String, String, Vec<String>)>,
    no_pre: bool,
    no_unit_test: bool,
    no_build: bool,
    no_functional_test: bool,
    no_deploy: bool,
    no_system_test: bool,
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
    pub fn set_pre_script(&mut self, pre_script: String) {
        self.pre_script = pre_script;
    }
    pub fn set_pre_env(&mut self, env_options: Vec<(String, String, String, String, Vec<String>)>) {
        self.pre_env = env_options;
    }
    pub fn set_no_pre(&mut self, setting: bool) {
        self.no_pre = setting;
    }
    pub fn set_unit_test(&mut self, script: String) {
        self.unit_test = script;
    }
    pub fn set_unit_test_env(&mut self, env_options: Vec<(String, String, String, String, Vec<String>)>) {
        self.unit_test_env = env_options;
    }
    pub fn set_build_script(&mut self, script: String) {
        self.build_script = script;
    }
    pub fn set_build_env(&mut self, env_options: Vec<(String, String, String, String, Vec<String>)>) {
        self.build_env = env_options;
    }
    pub fn set_functional_test(&mut self, script: String) {
        self.functional_test = script;
    }
    pub fn set_functional_test_env(&mut self, env_options: Vec<(String, String, String, String, Vec<String>)>) {
        self.functional_test_env = env_options;
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
    pub fn set_system_test(&mut self, script: String) {
        self.system_test = script;
    }
    pub fn set_system_test_env(&mut self, env_options: Vec<(String, String, String, String, Vec<String>)>) {
        self.system_test_env = env_options;
    }
    pub fn set_post_script(&mut self, post_script: String) {
        self.post_script = post_script;
    }
    pub fn set_post_env(&mut self, env_options: Vec<(String, String, String, String, Vec<String>)>) {
        self.post_env = env_options;
    }
    pub fn set_no_unit_test(&mut self, setting: bool) {
        self.no_unit_test = setting;
    }
    pub fn set_no_build(&mut self, setting: bool) {
        self.no_build = setting;
    }
    pub fn set_no_functional_test(&mut self, setting: bool) {
        self.no_functional_test = setting;
    }
    pub fn set_no_deploy(&mut self, setting: bool) {
        self.no_deploy = setting;
    }
    pub fn set_no_template(&mut self, setting: bool) {
        self.no_template = setting;
    }
    pub fn set_no_system_test(&mut self, setting: bool) {
        self.no_system_test = setting;
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
    pub fn pre_script(&self) -> &String {
        &self.pre_script
    }
    pub fn pre_env(&self) -> &Vec<(String, String, String, String, Vec<String>)> {
        &self.pre_env
    }
    pub fn unit_test(&self) -> &String {
        &self.unit_test
    }
    pub fn unit_test_env(&self) -> &Vec<(String, String, String, String, Vec<String>)> {
        &self.unit_test_env
    }
    pub fn build_script(&self) -> &String {
        &self.build_script
    }
    pub fn functional_test(&self) -> &String {
        &self.functional_test
    }
    pub fn functional_test_env(&self) -> &Vec<(String, String, String, String, Vec<String>)> {
        &self.functional_test_env
    }
    pub fn system_test(&self) -> &String {
        &self.system_test
    }
    pub fn system_test_env(&self) -> &Vec<(String, String, String, String, Vec<String>)> {
        &self.system_test_env
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
    pub fn has_pre_script(&self) -> bool {
        match self.pre_script.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
    pub fn has_no_pre(&self) -> bool {
        self.no_pre
    }
    pub fn has_build_script(&self) -> bool {
        match self.build_script.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
    pub fn has_unit_test(&self) -> bool {
        match self.unit_test.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
    pub fn has_system_test(&self) -> bool {
        match self.system_test.len()  {
            n if n > 0 => true,
            _ => false,
        }
    }
    pub fn has_functional_test(&self) -> bool {
        match self.functional_test.len()  {
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
    pub fn has_no_unit_test(&self) -> bool {
        self.no_unit_test
    }
    pub fn has_no_build(&self) -> bool {
        self.no_build
    }
    pub fn has_no_functional_test(&self) -> bool {
        self.no_functional_test
    }
    pub fn has_no_deploy(&self) -> bool {
        self.no_deploy
    }
    pub fn has_no_system_test(&self) -> bool {
        self.no_system_test
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
        write!(f, "Configs {{\n\t template: {}\n\t params: {}\n\t params-script: {}\n\t pre-script: {}\n\t unit-test: {}\n\t build-script: {}\n\t functional-test: {}\n\t deploy-script: {}\n\t system-test: {}\n\t post-script: {}\n}}",
        self.template, self.params, self.param_script, self.pre_script, self.unit_test, self.build_script, self.functional_test, self.deploy_script, self.system_test, self.post_script)
    }
}