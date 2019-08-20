use std::path::PathBuf;
use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};

#[derive(Serialize, Debug, Default, Clone)]
pub struct Dockerization {
    pub image: Option<String>,
    pub dockerfile: Option<PathBuf>,
    pub port: Option<i32>,
    pub volumes: Option<Vec<String>>,
}

impl Dockerization {
    pub fn new() -> Self  {
        Default::default()
    }
    
    pub fn new_values(image: Option<String>, dockerfile: Option<PathBuf>, port: Option<i32>, volumes: Option<Vec<String>>) -> Self  {
        Dockerization {
            image,
            dockerfile,
            port,
            volumes,
        }
    }
}

impl<'de> Deserialize<'de> for Dockerization {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Image, Dockerfile, Port, Volumes };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`image` or `dockerfile` or `port` or `volumes`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "image" => Ok(Field::Image),
                            "dockerfile" => Ok(Field::Dockerfile),
                            "port" => Ok(Field::Port),
                            "volumes" => Ok(Field::Volumes),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        
        struct DockerizationVisitor;

        impl<'de> Visitor<'de> for DockerizationVisitor {
            type Value = Dockerization;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Dockerization")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Dockerization, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut image = None;
                let mut dockerfile = None;
                let mut port = None;
                let mut volumes = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Image => {
                            if image.is_some() {
                                return Err(de::Error::duplicate_field("image"));
                            }
                            image = Some(map.next_value()?);
                        }
                        Field::Port => {
                            if port.is_some() {
                                return Err(de::Error::duplicate_field("port"));
                            }
                            port = Some(map.next_value()?);
                        }
                        Field::Volumes => {
                            if volumes.is_some() {
                                return Err(de::Error::duplicate_field("volumes"));
                            }
                            volumes = Some(map.next_value()?);
                        }
                        Field::Dockerfile => {
                            if dockerfile.is_some() {
                                return Err(de::Error::duplicate_field("path"));
                            }
                            dockerfile = match map.next_value() {
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
                
                let volumes = volumes.unwrap_or_else(|| None);
                let dockerfile = match &dockerfile {
                    Some(x) => {
                        let new_path = std::fs::canonicalize(x.as_path());
                        match new_path {
                            Ok(y) => { 
                                Some(y)
                            },
                            Err(e) => {eprintln!("Error loading {:?}: {:?}", dockerfile.clone().unwrap(), e.kind()); dockerfile}
                        }
                    },
                    None => {None},
                };
                
                Ok(Dockerization::new_values(image, dockerfile, port, volumes))
            }
        }
        
        const FIELDS: &'static [&'static str] = &["image", "dockerfile", "port", "volumes"];
        deserializer.deserialize_struct("Dockerization", FIELDS, DockerizationVisitor)
    }
}

