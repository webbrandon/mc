use super::env_prompt::EnvironmentPrompt;
use super::template::Template;

use std::path::PathBuf;
use std::collections::HashMap;
use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};

#[serde(rename_all = "kebab-case")]
#[derive(Serialize, Debug, Default, Clone)]
pub struct Step {
    pub order: Option<usize>,
    pub env_prompt: Option<Vec<EnvironmentPrompt>>,
    pub script: Option<PathBuf>,
    pub templates: Option<Vec<Template>>,
    pub post_script: Option<PathBuf>,
}

impl Step {
    pub fn new() -> Step {
        Step {
            order: None,
            env_prompt: Some(vec![EnvironmentPrompt::new()]),
            script: None,
            templates: Some(vec![Template::new()]),
            post_script: None,
        }
    }
    
    pub fn new_values(
        order: Option<usize>, env_prompt: Option<Vec<EnvironmentPrompt>>,
        script: Option<PathBuf>, templates: Option<Vec<Template>>,
        post_script: Option<PathBuf>) -> Step 
    {
        Step {
            order,env_prompt,script,templates,post_script
        }
    }
    
    pub fn default_set(self) -> HashMap<String, Step> {
        let mut steps = HashMap::new();
        let mut step = Step::new();
        let template = Template::new();
        step.templates = Some(vec![template]);
        
        steps.insert(String::from("pre"), step.clone());
        steps.insert(String::from("unit-test"), step.clone());
        steps.insert(String::from("build"), step.clone());
        steps.insert(String::from("template"), step.clone());
        steps.insert(String::from("deploy"), step.clone());
        steps.insert(String::from("functional-test"), step.clone());
        steps.insert(String::from("system-test"), step.clone());
        steps.insert(String::from("post"), step.clone());
        
        steps
    }
    
    pub fn new_empty() -> Step {
        Default::default()
    }
}


impl<'de> Deserialize<'de> for Step {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Order, Prompt, Script, Temp, Post };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`order` or `env-prompt` or `script` or `templates` or `post-script`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "order" => Ok(Field::Order),
                            "env-prompt" => Ok(Field::Prompt),
                            "script" => Ok(Field::Script),
                            "templates" => Ok(Field::Temp),
                            "post-script" => Ok(Field::Post),
                            _ => Err(de::Error::custom("Uknown field name.")),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        
        struct StepVisitor;

        impl<'de> Visitor<'de> for StepVisitor {
            type Value = Step;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Step")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Step, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut order = None;
                let mut env_prompt = None;
                let mut script = None;
                let mut templates = None;
                let mut post_script = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Order => {
                            if order.is_some() {
                                return Err(de::Error::duplicate_field("order"));
                            }
                            order = Some(map.next_value()?);
                        }
                        Field::Prompt => {
                            if env_prompt.is_some() {
                                return Err(de::Error::duplicate_field("env-prompt"));
                            }
                            env_prompt = Some(map.next_value()?);
                        }
                        Field::Script => {
                            if script.is_some() {
                                return Err(de::Error::duplicate_field("script"));
                            }
                            script = match map.next_value() {
                                Ok(x) => {
                                    match x {
                                        Some(y) => {
                                            let new_path: PathBuf = y;
                                            Some(std::fs::canonicalize(new_path.as_path()).unwrap_or(new_path))
                                        },
                                        None => {None},
                                    }
                                },
                                Err(e) => {eprintln!("{:?}", e);None},
                            };
                        }
                        Field::Temp => {
                            if templates.is_some() {
                                return Err(de::Error::duplicate_field("template"));
                            }
                            templates = Some(map.next_value()?);
                        }
                        Field::Post => {
                            if post_script.is_some() {
                                return Err(de::Error::duplicate_field("post-script"));
                            }
                            post_script = match map.next_value() {
                                Ok(x) => {
                                    match x {
                                        Some(y) => {
                                            let new_path: PathBuf = y;
                                            Some(std::fs::canonicalize(new_path.as_path()).unwrap_or(new_path))
                                        },
                                        None => {None},
                                    }
                                },
                                Err(e) => {eprintln!("{:?}", e);None},
                            };
                        }
                    }
                }
                
                let order = order.unwrap_or_else(|| None);
                let env_prompt = env_prompt.unwrap_or_else(|| None);
                let templates = templates.unwrap_or_else(|| None);
                let script = match &script {
                    Some(x) => {
                        let new_path = std::fs::canonicalize(x.as_path());
                        match new_path {
                            Ok(y) => { 
                                Some(y)
                            },
                            Err(e) => {eprintln!("Error loading {:?}: {:?}", script.clone().unwrap(), e.kind()); script}
                        }
                    },
                    None => {None},
                };
                let post_script = match &post_script {
                    Some(x) => {
                        let new_path = std::fs::canonicalize(x.as_path());
                        match new_path {
                            Ok(y) => { 
                                Some(y)
                            },
                            Err(e) => {eprintln!("Error loading {:?}: {:?}", post_script.clone().unwrap(), e.kind()); post_script}
                        }
                    },
                    None => {None},
                };
                
                Ok(Step::new_values(order, env_prompt, script, templates, post_script))
            }
        }
        
        const FIELDS: &'static [&'static str] = &["order", "env-prompt", "script", "templates", "post-script"];
        deserializer.deserialize_struct("Step", FIELDS, StepVisitor)
    }
}
