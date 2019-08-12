use crate::models::repository::Repository;
use git2;
use std::env;
use std::fs;
use std::path::PathBuf;

/// The EnvironmentFile load a dot env file. (eg: .env)
#[derive(Debug, Default)]
pub struct RepositoryHandler {}

impl RepositoryHandler {
    pub fn process(repo_model: Repository) {
        match repo_model.url {
            Some(repo_url) => {
                let dir = RepositoryHandler::clone_path(repo_url.clone());

                RepositoryHandler::create_dir(dir.clone());
                RepositoryHandler::clone_repo(repo_url.clone(), dir.clone());
                RepositoryHandler::enter_clone_dir(dir.clone(), repo_model.path);
            }
            None => { /* Do nothing. */ }
        }
    }

    pub fn enter_clone_dir(dir: String, path: Option<PathBuf>) -> bool {
        match path {
            Some(path) => env::set_current_dir(path).is_ok(),
            None => env::set_current_dir(&dir).is_ok(),
        }
    }

    pub fn clone_repo(repo_url: String, path: String) -> bool {
        // Clone repo if it doesn't exist.
        match git2::Repository::clone(&repo_url, &path) {
            Ok(_repo) => {
                println!("Cloned repository: {:?}", repo_url);
                true
            }
            Err(_e) => false,
        }
    }

    pub fn clone_path(repo_url: String) -> String {
        // Kinda long and conplicated way of getting the path name.  Come back.
        let url_seg_iter = repo_url.split("/").to_owned();
        let url_seg = url_seg_iter.collect::<Vec<&str>>();
        let name_seg = url_seg[url_seg.len() - 1].split(".git").to_owned();
        let name = name_seg.collect::<Vec<&str>>();
        format!("{}/{}", env::var("PWD").unwrap_or("".to_string()), name[0])
    }

    pub fn create_dir(path: String) -> bool {
        // Create a directory for repository if it doesn't already exist.
        match fs::create_dir(&path) {
            Ok(_dir) => {
                println!("Created directory for repository.\n Location: {}", path);
                true
            }
            Err(_e) => {
                /* Come back and see if error is thrown if dir exist */
                false
            }
        }
    }
}
