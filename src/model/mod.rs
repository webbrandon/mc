use std::fmt;
use file::parse_json;
use std::process;
use handlebars::Handlebars;
use run_script;
use run_script::ScriptOptions;
use file::bad_format_close_app;
use file::file_to_string;

pub fn run_build_script(script: &String) -> String {
    let mut options = ScriptOptions::new();
    options.runner = None; 
    options.capture_output = true; 
    options.exit_on_error = false; 
    options.print_commands = false;

    let args = vec![];

    let (code, output, error) = run_script::run(
        script,
        &args,
        &options,
    ).unwrap();
    if code == 0 {
        output
    } else {
        println!("{}", error);
        bad_format_close_app();
    }
}
    
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

