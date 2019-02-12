use crate::model::{Configs};
use std::fs;
use std::env;
use git2::Repository;

pub struct Repo {}

impl Repo {
    pub fn check_and_process(request: &Configs) {
        if request.has_repository() {
            let repo_url = request.repository();
            
            // Kinda long and conplicated way of getting the path name.  Come back.
            let url_segments_iter = repo_url.split("/").to_owned();
            let url_segments = url_segments_iter.collect::<Vec<&str>>();
            let git_name_segments = url_segments[ url_segments.len() - 1 ].split(".git").to_owned();
            let git_name = git_name_segments.collect::<Vec<&str>>();
            let git_dir = format!("{}/{}", env::var("PWD").unwrap_or("/opt/repo".to_string()), git_name[0]);
            
            // Create a directory for repository if it doesn't already exist.
            match fs::create_dir(&git_dir) {
                Ok(_repo) => {},
                Err(_e) => {},
            }
            
            // Clone repo if it doesn't exist.
            match Repository::clone(repo_url, &git_dir) {
                Ok(_repo) => {
                    env::set_current_dir(&git_dir).is_ok();
                },
                Err(_e) => {
                    env::set_current_dir(&git_dir).is_ok();
                },
            }

            
        }
    }
}








