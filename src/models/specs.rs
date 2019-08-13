use super::env_file::EnvironmentFile;
use super::env_prompt::EnvironmentPrompt;
use super::flows::Flow;
use super::repository::Repository;
use super::steps::Step;
use std::collections::HashMap;
use std::path::PathBuf;

#[serde(rename_all = "kebab-case")]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Specs {
    pub repository: Option<Repository>,
    pub steps: HashMap<String, Step>,
    pub env_file: Option<EnvironmentFile>,
    pub env_prompt: Option<Vec<EnvironmentPrompt>>,
    pub flows: Option<Vec<Flow>>,
}

impl Specs {
    pub fn new() -> Specs {
        Specs {
            repository: Some(Repository::new()),
            steps: Step::new().default_set(),
            env_file: Some(EnvironmentFile::new()),
            env_prompt: Some(vec![EnvironmentPrompt::new()]),
            flows: Some(vec![Flow::new()]),
        }
    }
    
    pub fn set_repo_url(&mut self, url: String) {
        let repo = Repository {
            path: None,
            url: Some(url),
        };
        self.repository = Some(repo);
    }

    pub fn set_env_file_path(&mut self, path: PathBuf) {
        let env_file = EnvironmentFile {
            path: Some(path),
            url: None,
        };
        self.env_file = Some(env_file);
    }

    pub fn set_flow(&mut self, mut option: &str) {
        if option.is_empty() {
            option = "default";
        }

        match self.flows.clone() {
            Some(x) => {
                for flow in x {
                    if flow.name == option {
                        self.flows = Some(vec![flow])
                    }
                }
            }
            None => {}
        }
    }

    pub fn default_sort(&mut self) -> Vec<(String, Step)> {
        let mut sorted_steps: Vec<(String, Step)> = Vec::new();
        let mut steps_copy = self.steps.clone();
        let step_order_instructions = vec![
            "pre",
            "unit-test",
            "build",
            "functional-test",
            "template",
            "deploy",
            "system-test",
            "post",
        ];

        for step_order in step_order_instructions {
            if steps_copy.contains_key(step_order) {
                match steps_copy.get(step_order) {
                    Some(x) => {
                        sorted_steps.push((step_order.to_string(), x.clone()));
                        steps_copy.remove(step_order);
                    }
                    None => {}
                }
            }
        }

        //  Order steps by our defined default pipeline order.  Track custom step request for later.
        let custom_steps: Vec<(String, Step)> = steps_copy.clone().into_iter().collect();

        // Add custom step elements.  Insert by order if requested and will be based on
        // available defined steps in a default pipeline order or else push to end in order
        // recieved.  We should rethink this structure as custom event will more than likely
        // always be random due to the nature of HashMap.
        for step_model in custom_steps {
            match step_model.1.order {
                Some(order) => {
                    if sorted_steps.len() > order {
                        sorted_steps.insert(order - 1, step_model);
                    }
                }
                None => {
                    sorted_steps.push(step_model);
                }
            }
        }

        sorted_steps
    }

    pub fn flow_sort(&mut self, flow: Flow) -> Vec<(String, Step)> {
        let mut sorted_steps: Vec<(String, Step)> = Vec::new();

        for step in flow.flow {
            let _get_sorted: Vec<_> = self
                .steps
                .clone()
                .into_iter()
                .map(|(name, step_model)| {
                    if name == step {
                        sorted_steps.push((name, step_model));
                    }
                })
                .collect();
        }

        sorted_steps
    }

    pub fn sort_steps(&mut self) -> Vec<(String, Step)> {
        match &self.flows {
            Some(list) => {
                if list.len() > 1 || list.is_empty() {
                    eprintln!("There was a problem selecting flow.");
                    self.default_sort()
                } else {
                    self.clone().flow_sort(list[0].to_owned())
                }
            }
            None => self.default_sort(),
        }
    }
}
