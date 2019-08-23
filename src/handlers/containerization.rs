extern crate bollard;
extern crate tokio;
extern crate futures;

use futures::future::Future;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::prelude::*;
use tokio::runtime::Runtime;
use bollard::container::{
    Config, CreateContainerOptions, StartContainerOptions, HostConfig
};
use bollard::image::{BuildImageOptions, CreateImageOptions};
use bollard::Docker;
use crate::models::{Containerization};

/// The EnvironmentFile load a dot env file. (eg: .env)
#[derive(Debug, Default, Clone)]
pub struct ContainerizationHandler {
    pub current_container: String,
    pub build_file: PathBuf,
    pub container_model: Containerization,
}

impl ContainerizationHandler {
    pub fn new() -> Self {
        Default::default()
    }
    
    pub fn process(&mut self) {
        match &self.container_model.docker {
            Some(docker_model) => {
                match &docker_model.image {
                    Some(_image) => self.docker_run_slave(),
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
                        self.current_container = image;
                    },
                    None => {
                        self.current_container = String::from("mc-slave:local");
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
    
    pub fn docker_connect(&mut self) -> Docker {
        let docker = Docker::connect_with_local_defaults().unwrap();
        #[cfg(feature = "tls")]
        let docker = Docker::connect_with_tls_defaults().unwrap();
        docker
    }
    
    pub fn docker_build_config(&mut self) -> BuildImageOptions<String> {
        let mut build_image_args = HashMap::new();
        build_image_args.insert("curl https://webbrandon.github.io/mc/install.sh -sS | bash -s".to_string(), "".to_string());

        let mut build_image_labels = HashMap::new();
        build_image_labels.insert("app".to_string(), "MasterOfCeremony".to_string());
        let tag = self.current_container.clone();
        
        BuildImageOptions {
            dockerfile: self.build_file.to_str().unwrap_or("").to_string(),
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
        
        let future = docker
            .build_image(build_config, None, None)
            .map(|v| {
                println!("{:?}", v);
                v
            })
            .collect()
            .map_err(|e| {
                println!("{:?}", e);
                ()
            })
            .map(|_| ());

        rt.spawn(future);

        rt.shutdown_on_idle().wait().unwrap();
    }
    
    pub fn docker_run_config(&mut self) -> Config<String> {
        Config {
            image: Some(self.current_container.clone()),
            host_config: Some(HostConfig {
                network_mode: Some("host".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
    
    pub fn docker_run_slave(&mut self) {
        let mut rt = Runtime::new().unwrap();
        let docker = self.docker_connect();
        let run_config = self.docker_run_config();
        
        let stream = docker
            .chain()
            .create_image(
                Some(CreateImageOptions {
                    from_image: self.current_container.clone(),
                    ..Default::default()
                }),
                None,
            )
            .and_then(move |(docker, _)| {
                docker.create_container(
                    Some(CreateContainerOptions { name: "mc" }),
                    run_config,
                )
            })
            .and_then(move |(docker, _)| {
                docker.start_container("mc", None::<StartContainerOptions<String>>)
            })
            .into_stream();
            
        let future = stream
            .map_err(|e| println!("{:?}", e))
            .for_each(|x| Ok(println!("{:?}", x)));
        
        rt.spawn(future);

        rt.shutdown_on_idle().wait().unwrap();
    }
}
