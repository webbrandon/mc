use std::path::PathBuf;
use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};

#[derive(Serialize, Debug, Default, Clone)]
pub struct Repository {
    pub url: Option<String>,
    pub path: Option<PathBuf>,
}

impl Repository {
    pub fn new() -> Repository {
        Default::default()
    }
    
    pub fn new_values(url: Option<String>, path: Option<PathBuf>) -> Repository  {
        Repository {
            url,
            path,
        }
    }
}

impl<'de> Deserialize<'de> for Repository {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Path, Url };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`path` or `url`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "url" => Ok(Field::Url),
                            "path" => Ok(Field::Path),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        
        struct RepositoryVisitor;

        impl<'de> Visitor<'de> for RepositoryVisitor {
            type Value = Repository;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Repository")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Repository, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut url = None;
                let mut path = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Url => {
                            if url.is_some() {
                                return Err(de::Error::duplicate_field("url"));
                            }
                            url = Some(map.next_value()?);
                        }
                        Field::Path => {
                            if path.is_some() {
                                return Err(de::Error::duplicate_field("path"));
                            }
                            path = match map.next_value() {
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
                
                let url = url.unwrap_or_else(|| None);
                let path = match &path {
                    Some(x) => {
                        let new_path = std::fs::canonicalize(x.as_path());
                        match new_path {
                            Ok(y) => { 
                                Some(y)
                            },
                            Err(e) => {eprintln!("Error loading {:?}: {:?}", path.clone().unwrap(), e.kind()); path}
                        }
                    },
                    None => {None},
                };
                
                Ok(Repository::new_values(url, path))
            }
        }
        
        const FIELDS: &'static [&'static str] = &["path", "url"];
        deserializer.deserialize_struct("Repository", FIELDS, RepositoryVisitor)
    }
}
