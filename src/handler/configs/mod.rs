pub use model::{Configs};

use file::file_to_string;
use yaml_rust::yaml;
use clap::{ArgMatches};

impl Configs {
    pub fn process_args(matches: &ArgMatches) -> Configs {
        let mut request = Configs::new();
        if (matches.is_present("no-build") || matches.is_present("no-deploy")) || matches.is_present("no-template") || matches.is_present("no-post") || matches.is_present("no-prompt") {
            request.set_no_build(matches.is_present("no-build"));
            request.set_no_deploy(matches.is_present("no-deploy"));
            request.set_no_template(matches.is_present("no-template"));
            request.set_no_post(matches.is_present("no-post"));
            request.set_no_prompt(matches.is_present("no-prompt"));
        }
        if (matches.is_present("template") && matches.is_present("parameters")) || matches.is_present("script") {
            request.set_template(matches.value_of("template").unwrap_or("").to_owned());
            request.set_param_script(matches.value_of("param-script").unwrap_or("").to_owned());
            request.set_params(matches.value_of("parameters").unwrap_or("").to_owned());
            request.set_render(matches.value_of("template-out").unwrap_or("").to_owned());
            request.set_build_script(matches.value_of("build-script").unwrap_or("").to_owned());
            request.set_deploy_script(matches.value_of("deploy-script").unwrap_or("").to_owned());
            request.set_post_script(matches.value_of("post-script").unwrap_or("").to_owned());
        } else {
            let makeconfig = matches.value_of("config").unwrap_or("Makeconfig.yaml");
            let s = file_to_string(makeconfig);
            let docs = yaml::YamlLoader::load_from_str(&s).unwrap();
            for doc in &docs {
                request.set_template(doc["specs"]["template"]["file"].as_str().unwrap_or("").to_owned());
                request.set_param_script(doc["specs"]["parameters"]["create"].as_str().unwrap_or("").to_owned());
                request.set_render(doc["specs"]["template-out"]["file"].as_str().unwrap_or("").to_owned());
                request.set_build_script(doc["specs"]["build-script"]["file"].as_str().unwrap_or("").to_owned());
                request.set_deploy_script(doc["specs"]["build-script"]["file"].as_str().unwrap_or("").to_owned());
                request.set_post_script(doc["specs"]["post-script"]["file"].as_str().unwrap_or("").to_owned());
                
                let param_location = doc["specs"]["parameters"]["type"].as_str().unwrap_or("").to_owned();
                if param_location == "Url".to_string() {
                    let mut url:String = String::new();
                    let remote_check = &doc["specs"]["parameters"]["host"].as_str().unwrap_or("").to_owned()[..4].to_string();
                    if ! remote_check.contains("http") {
                        url.push_str("http://");
                    } 
                    url.push_str(&doc["specs"]["parameters"]["host"].as_str().unwrap_or("").to_owned());
                    url.push_str(&doc["specs"]["parameters"]["path"].as_str().unwrap_or("").to_owned());
                    request.set_params(url);
                } else if param_location == "File".to_string() {
                    request.set_params(doc["specs"]["parameters"]["path"].as_str().unwrap_or("").to_owned());
                }
            }
        }
        request
    }
}