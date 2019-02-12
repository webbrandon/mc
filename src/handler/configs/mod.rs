pub use crate::model::{Configs};

//use std::process;
use crate::file::file_to_string;
use yaml_rust::yaml;
use clap::{ArgMatches};

fn extract_env(yaml_doc: yaml_rust::Yaml) -> Vec<(String, String, String, String, Vec<String>)> {
    let mut env_list:Vec<(String, String, String, String, Vec<String>)> = Vec::new();
    
    for x in yaml_doc {    
        let name: String;
        let type_value: String;
        let context: String;
        let default_value: String;
        
        match x["name"].as_str() {
            Some(a) => {name = a.to_string();},
            None => {name = String::new();}
        }
        match x["type"].as_str() {
            Some(a) => {type_value = a.to_string();},
            None => {type_value = "input".to_string();}
        }
        match x["context"].as_str() {
            Some(a) => {context = a.to_string();},
            None => {context = "".to_string();}
        }
        match x["default"].as_str() {
            Some(a) => {default_value = a.to_string()},
            None => {default_value = String::new();}
        }
        let mut value_options: Vec<String> = Vec::new();
        match x["options"].as_vec() {
            Some(a) => {
                for b in a {
                    value_options.push(b["value"].as_str().unwrap().to_string());
                }
            },
            None => {value_options = Vec::new();}
        }
        env_list.push((name, type_value, context, default_value, value_options));
    }
    env_list    
}

pub fn flow_by_type(flow_name: String, yaml_doc: yaml_rust::Yaml) -> (String, String, Vec<String>) {
    let mut flow: Vec<String> = Vec::new();
    let mut flow_env = String::new();
    
    for x in yaml_doc {
        if x["name"].as_str() == Some(flow_name.as_str()) {
            match &x["env-file"].as_str() {
                Some(a) => {
                    flow_env = a.to_string();
                },
                None => {
                    println!("Empty flow pattern.  Will apply default pattern.");
                }
            }
            match x["flow"].as_vec() {
                Some(a) => {
                    for b in a {
                        match b.as_str() {
                            Some(z) => {
                                flow.push(z.to_string());
                            },
                            None    => print!("You cannot have empty steps in flow request.")
                        }
                    }
                },
                None => {
                    println!("Empty flow pattern.  Will apply default pattern.");
                }
            }
        }
    }
    (flow_name, flow_env, flow)
}
    
impl Configs {
    pub fn process_args(matches: ArgMatches) -> Configs {
        let mut request = Configs::new();
        if (matches.is_present("template") && matches.is_present("parameters")) || matches.is_present("script") {
            request.set_template(matches.value_of("template").unwrap_or("").to_owned());
            request.set_param_script(matches.value_of("param-script").unwrap_or("").to_owned());
            request.set_params(matches.value_of("parameters").unwrap_or("").to_owned());
            request.set_render(matches.value_of("template-out").unwrap_or("").to_owned());
            request.set_repository(matches.value_of("repo").unwrap_or("").to_owned());
            request.set_pre_script(matches.value_of("pre-script").unwrap_or("").to_owned());
            request.set_unit_test(matches.value_of("unit-test").unwrap_or("").to_owned());
            request.set_build_script(matches.value_of("build-script").unwrap_or("").to_owned());
            request.set_functional_test(matches.value_of("functional-test").unwrap_or("").to_owned());
            request.set_deploy_script(matches.value_of("deploy-script").unwrap_or("").to_owned());
            request.set_system_test(matches.value_of("system-test").unwrap_or("").to_owned());
            request.set_post_script(matches.value_of("post-script").unwrap_or("").to_owned());
        } else {
            let mc_config = matches.value_of("config").unwrap_or("mc.yaml");
            let s = file_to_string(mc_config);
            let docs = yaml::YamlLoader::load_from_str(&s).unwrap();
            for doc in &docs {
                if (matches.is_present("flow") || matches.is_present("no-pre") || matches.is_present("no-build") || matches.is_present("no-deploy")) || matches.is_present("no-template") || matches.is_present("no-post") || matches.is_present("no-prompt") {
                    request.set_no_pre(matches.is_present("no-pre"));
                    request.set_no_unit_test(matches.is_present("no-unit-test"));
                    request.set_no_build(matches.is_present("no-build"));
                    request.set_no_functional_test(matches.is_present("no-functional-test"));
                    request.set_no_deploy(matches.is_present("no-deploy"));
                    request.set_no_system_test(matches.is_present("no-system-test"));
                    request.set_no_template(matches.is_present("no-template"));
                    request.set_no_post(matches.is_present("no-post"));
                    request.set_no_prompt(matches.is_present("no-prompt"));
                    request.set_flow_name(matches.value_of("flow").unwrap_or("").to_owned());
                    request.set_flow(flow_by_type(request.flow_name().to_string(), doc["specs"]["flows"].to_owned()));
                }
                request.set_repository(doc["specs"]["repository"]["url"].as_str().unwrap_or("").to_owned());
                request.set_template(doc["specs"]["steps"]["template"]["file"].as_str().unwrap_or("").to_owned());
                request.set_param_script(doc["specs"]["steps"]["template"]["script"].as_str().unwrap_or("").to_owned());
                request.set_render(doc["specs"]["steps"]["template"]["outfile"].as_str().unwrap_or("").to_owned());
                request.set_pre_script(doc["specs"]["steps"]["pre-script"]["file"].as_str().unwrap_or("").to_owned());
                request.set_unit_test(doc["specs"]["steps"]["unit-test"]["file"].as_str().unwrap_or("").to_owned());
                request.set_build_script(doc["specs"]["steps"]["build-script"]["file"].as_str().unwrap_or("").to_owned());
                request.set_functional_test(doc["specs"]["steps"]["functional-test"]["file"].as_str().unwrap_or("").to_owned());
                request.set_deploy_script(doc["specs"]["steps"]["deploy-script"]["file"].as_str().unwrap_or("").to_owned());
                request.set_system_test(doc["specs"]["steps"]["system-test"]["file"].as_str().unwrap_or("").to_owned());
                request.set_post_script(doc["specs"]["steps"]["post-script"]["file"].as_str().unwrap_or("").to_owned());
                
                // Check for enviornment name value settings.
                request.set_global_env(extract_env(doc["specs"]["env"].to_owned()));
                request.set_pre_env(extract_env(doc["specs"]["steps"]["pre-script"]["env"].to_owned()));
                request.set_unit_test_env(extract_env(doc["specs"]["steps"]["unit-test"]["env"].to_owned()));
                request.set_build_env(extract_env(doc["specs"]["steps"]["build-script"]["env"].to_owned()));
                request.set_functional_test_env(extract_env(doc["specs"]["steps"]["functional-test"]["env"].to_owned()));
                request.set_deploy_env(extract_env(doc["specs"]["steps"]["deploy-script"]["env"].to_owned()));
                request.set_system_test_env(extract_env(doc["specs"]["steps"]["system-test"]["env"].to_owned()));
                request.set_param_env(extract_env(doc["specs"]["steps"]["template"]["env"].to_owned()));
                request.set_post_env(extract_env(doc["specs"]["steps"]["post-script"]["env"].to_owned()));
                
                match doc["specs"]["steps"]["template"]["remote"].to_owned().as_str() {
                    Some(_a) => {
                        let mut url:String = String::new();
                        let remote_check = &doc["specs"]["steps"]["template"]["host"].as_str().unwrap_or("").to_owned()[..4].to_string();
                        if ! remote_check.contains("http") {
                            url.push_str("http://");
                        } 
                        url.push_str(&doc["specs"]["steps"]["template"]["host"].as_str().unwrap_or("").to_owned());
                        url.push_str(&doc["specs"]["steps"]["template"]["parameters"].as_str().unwrap_or("").to_owned());
                        request.set_params(url);
                    },
                    None => {
                        request.set_params(doc["specs"]["steps"]["template"]["parameters"].as_str().unwrap_or("").to_owned());
                    }
                }
            }
        }
        request
    }
}