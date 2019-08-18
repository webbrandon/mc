extern crate glob;

use std::fs;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{PathBuf};
use glob::glob;

#[derive(Debug, Default, Clone)]
pub struct MasterOfCeremonyFileHandler {
    
}

impl MasterOfCeremonyFileHandler {
    pub fn new() -> MasterOfCeremonyFileHandler {
        Default::default()
    }
    
    pub fn load(self, file: PathBuf) -> String {
        match self.read_into_string(file) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1)
            }
        }
    }

    pub fn read_into_string(self, file: PathBuf) -> Result<String, String> {
        let f = fs::read_to_string(file.clone());
        match f {
            Ok(x) => Ok(x),
            Err(e) => {
                Err(format!("Failed to open file: {:?}\nError: {}", file.to_str().unwrap(), e))
            }
        }
    }

    pub fn get_config_filenames(&mut self) -> Vec<String> {
        vec![
            String::from("mc.yaml"),
            String::from("mc.yml"),
            String::from("mc-flows.yaml"),
            String::from("mc-flows.yml"),
            String::from("mc-repo.yaml"),
            String::from("mc-repo.yml"),
            String::from("mc-steps.yaml"),
            String::from("mc-steps.yml"),
            String::from("mc-templates.yaml"),
            String::from("mc-templates.yml"),
            String::from("mc-env.yaml"),
            String::from("mc-env.yml"),
            String::from("mc-prompts.yaml"),
            String::from("mc-prompts.yml"),
            String::from("*.mc.yaml"),
            String::from("*.mc.yml"),
            String::from("*.mc-flows.yaml"),
            String::from("*.mc-flows.yml"),
            String::from("*.mc-repo.yaml"),
            String::from("*.mc-repo.yml"),
            String::from("*.mc-steps.yaml"),
            String::from("*.mc-steps.yml"),
            String::from("*.mc-templates.yaml"),
            String::from("*.mc-templates.yml"),
            String::from("*.mc-env.yaml"),
            String::from("*.mc-env.yml"),
            String::from("*.mc-prompts.yaml"),
            String::from("*.mc-prompts.yml"),
        ]
    }

    pub fn get_search_paths(&mut self) -> Vec<String> {
        vec![
            String::from("mc.yaml"),
            String::from("mc.yml"),
            String::from("*.mc.yaml"),
            String::from("*.mc.yml"),
            String::from("mc-*.yaml"),
            String::from("mc-*.yml"),
            String::from("*.mc-*.yaml"),
            String::from("*.mc-*.yml"),
        ]
    }

    pub fn get_file_map(&mut self) -> HashMap<String, Vec<PathBuf>> {
        let mut file_map: HashMap<String, Vec<PathBuf>> = HashMap::new();
        for path in self.get_config_filenames() {
            let mut precursor = String::from("./");
            precursor.push_str(&path);
            file_map.insert(path.clone(), vec![std::path::Path::new(&precursor).to_path_buf()]);
        }
        
        file_map
    }
    
    pub fn sort_file_paths(self, configs: HashMap<String, Vec<PathBuf>>) -> Vec<PathBuf> {
        let mut file_tree: Vec<PathBuf> = Vec::new();
        // preffered .yaml over .yml file first position (ie: 0) in tuple.
        let prefernce_comparisons = vec![
            ("mc.yaml", "mc.yml"),
            ("mc-flows.yaml", "mc-flows.yml"),
            ("mc-repo.yaml", "mc-repo.yml"),
            ("mc-steps.yaml", "mc-steps.yml"),
            ("mc-templates.yaml", "mc-templates.yml"),
            ("mc-prompts.yaml", "mc-prompts.yml"),
            ("mc-env.yaml", "mc-env.yml"),
        ];
        for comparison in &prefernce_comparisons {
            match (configs.get(comparison.0), configs.get(comparison.1)) {
                (Some(x), Some(_y)) => {
                    // ignore y when both exist
                    file_tree.push(x[0].to_owned());
                },
                (Some(x), None) => {
                    file_tree.push(x[0].to_owned());
                },
                (None, Some(x)) => {
                    file_tree.push(x[0].to_owned());
                },
                (None, None) => {},
            }
        }
        
        file_tree
    }

    pub fn calculate_file_map(mut self) -> Vec<PathBuf> {
        let config_file_map: HashMap<String, Vec<PathBuf>> = self.get_file_map();
        let mut filtered_file_map: HashMap<String, Vec<PathBuf>> = HashMap::new();
        
        for path in self.get_search_paths() {
            for e in glob(&path).expect("") {
                let file = e.unwrap().display().to_string();
                match config_file_map.get(&file) {
                    Some(x) => {
                        filtered_file_map.insert(file, x.to_vec());
                    },
                    None => {},
                }
            }
        }
        
        self.sort_file_paths(filtered_file_map)
    }

    pub fn get_config_paths(self, file: &Option<PathBuf>) -> Vec<PathBuf> {
        let config_map = match file {
            Some(x) => vec![x.to_path_buf()],
            None => self.calculate_file_map(),
        };
        
        config_map
    }
    
    pub fn out(self, filepath: PathBuf, text: &String) {
        let f = File::create(filepath).expect("Unable to create file");
        let mut f = BufWriter::new(f);
        f.write_all(text.as_bytes()).expect("Unable to write data");
    }
}