use crate::models::template::Template;
use super::mc_file;
use handlebars::Handlebars;
use serde_json::value::Value as Json;
use std::str::FromStr;

use mc_file::MasterOfCeremonyFileHandler;

/// The TemplateHandler will merge a template file with parameters file.
#[derive(Debug, Default, Clone)]
pub struct TemplateHandler {
    pub template: Vec<Template>,
    pub mute: bool,
}

impl TemplateHandler {
    pub fn new() -> TemplateHandler {
        Default::default()
    }

    pub fn process(&mut self) -> bool {
        for template in self.template.to_owned() {
            if !self.process_template(template) {
                std::process::exit(1)
            }
        }
        true
    }

    pub fn process_template(&mut self, template: Template) -> bool {
        let json = self.extract_json(MasterOfCeremonyFileHandler::load(MasterOfCeremonyFileHandler::new(), template.params.unwrap().to_owned()));
        let mut handlebars = Handlebars::new();

        handlebars
            .register_template_file("template", &template.template.unwrap().to_owned())
            .ok()
            .unwrap();

        match handlebars.render("template", &json) {
            Ok(data) => {
                self.std_out(data.clone());
                MasterOfCeremonyFileHandler::out(MasterOfCeremonyFileHandler::new(), template.out_file.unwrap().to_owned(), &data);
                true
            }
            Err(e) => {
                println!("Error rendering {}", e);
                std::process::exit(1)
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
                std::process::exit(1)
            }
        }
    }
}
