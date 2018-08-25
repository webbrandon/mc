pub use model::{Configs};

use file::file_to_string;
use yaml_rust::yaml;
use clap::{ArgMatches};

impl Configs {
    pub fn process_args(matches: &ArgMatches) -> Configs {
        let mut request = Configs::new();
        if (matches.is_present("template") && matches.is_present("parameters")) || matches.is_present("script") {
            request.set_template(matches.value_of("template").unwrap_or("").to_owned());
            request.set_params(matches.value_of("parameters").unwrap_or("").to_owned());
            request.set_render(matches.value_of("template-out").unwrap_or("").to_owned());
            request.set_script(matches.value_of("script").unwrap_or("").to_owned());
            request.set_post_script(matches.value_of("post-script").unwrap_or("").to_owned());
        } else {
            let makeconfig = matches.value_of("config").unwrap_or("Makeconfig.yaml");
            let s = file_to_string(makeconfig);
            let docs = yaml::YamlLoader::load_from_str(&s).unwrap();
            for doc in &docs {
                request.set_template(doc["specs"]["template"]["file"].as_str().unwrap_or("").to_owned());
                request.set_params(doc["specs"]["parameters"]["file"].as_str().unwrap_or("").to_owned());
                request.set_render(doc["specs"]["template-out"]["file"].as_str().unwrap_or("").to_owned());
                request.set_script(doc["specs"]["script"]["file"].as_str().unwrap_or("").to_owned());
                request.set_post_script(doc["specs"]["post-script"]["file"].as_str().unwrap_or("").to_owned());
            }
        }
        request
    }
}