use std::path::PathBuf;
use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};

#[serde(rename_all = "kebab-case")]
#[derive(Serialize, Debug, Default, Clone)]
pub struct Template {
    pub name: String,
    pub template: Option<PathBuf>,
    pub out_file: Option<PathBuf>,
    pub params: Option<PathBuf>,
}

impl Template {
    pub fn new() -> Template {
        Default::default()
    }
    
    pub fn new_values(
        name: String, template: Option<PathBuf>, 
        out_file: Option<PathBuf>, params: Option<PathBuf>) -> Template  
    {
        Template {
            name,
            template,
            out_file,
            params,
        }
    }
}


impl<'de> Deserialize<'de> for Template {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Name, Input, Output, Params };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`name` or `template` or `out-file` or `params`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "name" => Ok(Field::Name),
                            "template" => Ok(Field::Input),
                            "out-file" => Ok(Field::Output),
                            "params" => Ok(Field::Params),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        
        struct TemplateVisitor;

        impl<'de> Visitor<'de> for TemplateVisitor {
            type Value = Template;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Template")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Template, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut name = None;
                let mut template = None;
                let mut out_file = None;
                let mut params = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::Input => {
                            if template.is_some() {
                                return Err(de::Error::duplicate_field("path"));
                            }
                            template = match map.next_value() {
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
                        Field::Output => {
                            if out_file.is_some() {
                                return Err(de::Error::duplicate_field("path"));
                            }
                            out_file = match map.next_value() {
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
                        Field::Params => {
                            if params.is_some() {
                                return Err(de::Error::duplicate_field("path"));
                            }
                            params = match map.next_value() {
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
                
                let name = name.expect("");
                let template = match &template {
                    Some(x) => {
                        let new_path = std::fs::canonicalize(x.as_path());
                        match new_path {
                            Ok(y) => { 
                                Some(y)
                            },
                            Err(e) => {eprintln!("Error loading {:?}: {:?}", template.clone().unwrap(), e.kind()); template}
                        }
                    },
                    None => {None},
                };
                let out_file = match &out_file {
                    Some(x) => {
                        let new_path = std::fs::canonicalize(x.as_path());
                        match new_path {
                            Ok(y) => { 
                                Some(y)
                            },
                            Err(_e) => {out_file}
                        }
                    },
                    None => {None},
                };
                let params = match &params {
                    Some(x) => {
                        let new_path = std::fs::canonicalize(x.as_path());
                        match new_path {
                            Ok(y) => { 
                                Some(y)
                            },
                            Err(e) => {eprintln!("Error loading {:?}: {:?}", params.clone().unwrap(), e.kind()); params}
                        }
                    },
                    None => {None},
                };
                
                Ok(Template::new_values(name, template, out_file, params))
            }
        }
        
        const FIELDS: &'static [&'static str] = &["name", "template", "out-file", "params"];
        deserializer.deserialize_struct("Template", FIELDS, TemplateVisitor)
    }
}
