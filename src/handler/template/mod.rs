use crate::file::parse_json;
use std::process;
use handlebars::Handlebars;
pub use crate::model::{Template};

impl Template {
    pub fn render_template(&mut self) -> &String {
        let json = parse_json(&self.params());
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("template", &self.template())
            .ok()
            .unwrap();
        match handlebars.render("template", &json) {
            Ok(data) => {
                self.set_render(data);
                &self.render()
            }
            Err(e) => {
                println!("Error rendering {}", e);
                process::exit(2);
            }
        }
    }
}
