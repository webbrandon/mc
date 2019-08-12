use super::file;
use handlebars::Handlebars;
use serde_json::value::Value as Json;
use std::path::PathBuf;
use std::process;
use std::str::FromStr;

/// The TemplateHandler will merge a template file with parameters file.
#[derive(Debug, Default, Clone)]
pub struct TemplateHandler {
    pub template: PathBuf,
    pub out: PathBuf,
    pub params: PathBuf,
    pub mute: bool,
}

impl TemplateHandler {
    pub fn new() -> TemplateHandler {
        Default::default()
    }

    pub fn process(&mut self) -> bool {
        let json = self.extract_json(file::load(self.params.to_owned()));
        let mut handlebars = Handlebars::new();

        handlebars
            .register_template_file("template", &self.template)
            .ok()
            .unwrap();

        match handlebars.render("template", &json) {
            Ok(data) => {
                self.std_out(data.clone());
                file::out(self.out.to_owned(), &data);
                true
            }
            Err(e) => {
                println!("Error rendering {}", e);
                process::exit(1)
            }
        }
    }

    pub fn std_out(&mut self, content: String) {
        if !self.mute {
            println!("{:#?}", content);
        }
    }

    pub fn extract_json(&mut self, text: String) -> Json {
        match Json::from_str(&text) {
            Ok(json) => json,
            Err(e) => {
                println!("Error parsing file: {:#?}", e);
                process::exit(1);
            }
        }
    }
}
