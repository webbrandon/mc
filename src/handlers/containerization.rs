extern crate bollard;
extern crate tokio;
extern crate futures;
extern crate hyper;

use futures::future::Future;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::prelude::*;
use tokio::runtime::Runtime;
use bollard::container::{
    LogConfig, Config, CreateContainerOptions, StartContainerOptions, HostConfig, MountPoint, LogsOptions
};
use bollard::image::{BuildImageOptions, CreateImageOptions};
use bollard::{Docker};
use crate::models::{Containerization};

/// The EnvironmentFile load a dot env file. (eg: .env)
#[derive(Debug, Default, Clone)]
pub struct ContainerizationHandler {
    pub current_container: Vec<String>,
    pub build_file: PathBuf,
    pub volumes: Vec<MountPoint<String>>,
    pub container_model: Containerization,
}

impl ContainerizationHandler {
    pub fn new() -> Self {
        Default::default()
    }
    
    pub fn process(&mut self) {
        // Always make sure we add the base of operations.  This the base Path
        // for all volume paths.
        self.volumes.push(MountPoint {
            target: std::fs::canonicalize(Path::new(".")).unwrap().to_str().unwrap().to_string(),
            source: Path::new("/opt/mc").to_str().unwrap().to_string(),
            type_: "bind".to_string(),
            consistency: "default".to_string(),
            ..Default::default()
        });
        match &self.container_model.docker {
            Some(docker_model) => {
                match &docker_model.image {
                    Some(_image_name) => {
                        match self.docker_pull_slave() {
                            Ok(_image) => {
                                match self.docker_run_slave() {
                                    Ok(_) => {
                                        //println!("{:?}", run);
                                    },
                                    Err(e) => {
                                        println!("{:?}", e);
                                    }
                                }
                            },
                            Err(e) => {
                                println!("{:?}", e);
                            }
                        }
                    },
                    None => {
                        match &docker_model.dockerfile {
                            Some(_dockerfile) => self.build_docker_slave(),
                            None => {},
                        }
                    },
                }
            },
            None => {},
        }
    }
    
    pub fn set_container(&mut self, container_model: Containerization) {
        self.container_model = container_model.clone();
        match container_model.docker {
            Some(docker_model) => {
                match docker_model.image {
                    Some(image) => {
                        self.current_container = image.as_str().split(':').map(|s| s.to_string()).collect();
                    },
                    None => {
                        self.current_container = vec![String::from("mc-slave"),String::from("local")];
                    }
                }
                
                match docker_model.dockerfile {
                    Some(dockerfile) => {self.build_file = dockerfile;},
                    None => {},
                }
            },
            None => {},
        }
    }
    
    pub fn convert_to_container_path(&mut self, path: &PathBuf) -> String {
        let mut mutated_path:PathBuf; 
        let home_path = std::fs::canonicalize(Path::new(".")).unwrap();
        let slave_path = Path::new("/opt/mc");
        
        mutated_path = path.strip_prefix(&home_path).map_err(|_| ()).unwrap().to_path_buf();
        mutated_path = slave_path.join(mutated_path);
        
        mutated_path.to_str().unwrap().to_string()
    }
    
    pub fn set_volumes(&mut self, volumes: Vec<PathBuf>) {
        let mut mounts: Vec<MountPoint<String>> = Vec::new();
        for volume in volumes {
            
            mounts.push(MountPoint {
                target: self.convert_to_container_path(&volume),
                source: volume.to_str().unwrap_or("${PWD}").to_string(),
                type_: "bind".to_string(),
                consistency: "default".to_string(),
                ..Default::default()
            });
        }
        self.volumes = mounts;
    }
    
    pub fn docker_connect(&mut self) -> Docker {
        let docker = Docker::connect_with_local_defaults().unwrap();
        
        // #[cfg(feature = "tls")]
        // let docker = Docker::connect_with_tls_defaults().unwrap();
        docker
    }
    
    pub fn docker_build_config(&mut self) -> BuildImageOptions<String> {
        let mut build_image_args = HashMap::new();
        build_image_args.insert("curl https://webbrandon.github.io/mc/install.sh -sS | bash -s".to_string(), "".to_string());

        let mut build_image_labels = HashMap::new();
        build_image_labels.insert("app".to_string(), "MasterOfCeremony".to_string());
        let tag = self.current_container.join(":");
        
        BuildImageOptions {
            dockerfile: "Dockerfile.linux".to_string(),
            t: tag,
            q: true,
            nocache: false,
            pull: true,
            rm: true,
            forcerm: true,
            buildargs: build_image_args,
            labels: build_image_labels,
            ..Default::default()
        }
    }
    
    pub fn build_docker_slave(&mut self) {
        let mut rt = Runtime::new().unwrap();
        let docker = self.docker_connect();
        let build_config = self.docker_build_config();
        
        let stream = docker
            .chain()
            .build_image(build_config, None, None)
            .and_then(move |(docker, stream)| {
                stream.and_then(|l| {        
                    Ok(l)
                }).collect().map(|_| docker)
            })
            .into_stream();
            
        let future = stream
            .map_err(|e| println!("{:?}", e))
            .for_each(|_| {
                Ok(())
            });

        rt.spawn(future);

        rt.shutdown_on_idle().wait().unwrap();
    }
    
    pub fn docker_run_config(&mut self) -> Config<String> {
        let image = self.current_container.join(":").clone();
        Config {
            image: Some(image),
            tty: Some(true),
            attach_stdout: Some(true),
            open_stdin: Some(true),
            stdin_once: Some(true),
            attach_stdin: Some(true),
            attach_stderr: Some(true),
            cmd: Some(vec!["mc".to_string()]),
            host_config: Some(HostConfig {
                mounts: Some(self.volumes.clone()),
                log_config: Some(LogConfig {
                    type_: Some(String::from("json-file")),
                    config: Some(HashMap::new())
                }),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
    
    pub fn docker_pull_slave(&mut self) -> Result<(), ()> {
        let mut rt = Runtime::new().unwrap();
        let docker = self.docker_connect();
        
        let stream = docker
            .chain()
            .create_image(
                Some(CreateImageOptions {
                    from_image: self.current_container.join(":").clone(),
                    ..Default::default()
                }),
                None,
            )
            .and_then(move |(docker, stream)| {
                stream.and_then(|l| {        
                    Ok(l)
                }).collect().map(|_| docker)
            })
            .into_stream();
            
        let future = stream
            .map_err(|e| println!("{:?}", e))
            .for_each(|_| {
                Ok(())
            });
        
        rt.spawn(future);

        rt.shutdown_on_idle().wait()
    }
    
    pub fn docker_run_slave(&mut self) -> Result<(), ()> {
        let mut rt = Runtime::new().unwrap();
        let docker = self.docker_connect();
        let run_config = self.docker_run_config();
        let image_name: Vec<String> = self.current_container[0].as_str().split('/').map(|s| s.to_string()).collect();
        let name = image_name[image_name.len() - 1].clone();
        let stream = docker
            .chain()
            .create_container(
                Some(CreateContainerOptions { name: name.clone() }),
                run_config,
            )
            .and_then(move |(docker, _)| {
                docker.start_container(&name.clone(), None::<StartContainerOptions<String>>)
            })
            .and_then(move |(docker, _)| {
                let name = image_name[image_name.len() - 1].clone();
                docker.logs(
                    &name,
                    Some(LogsOptions {
                        follow: true,
                        stdout: true,
                        stderr: true,
                        ..Default::default()
                    }),
                )
            })
            .into_stream();
            
        let future = stream
            .map_err(|e| {
                println!("{:?}", e)
            })
            .for_each(|_| {
                Ok(())
            });
        
        rt.spawn(future);

        rt.shutdown_on_idle().wait()
    }
}
