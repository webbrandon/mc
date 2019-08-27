use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct ApiValidateHandler {}

impl ApiValidateHandler {
    /// Run validation against the api type and its version.
    pub fn valid_api_ver(api: String, version: String) -> bool {
        let mut apis = HashMap::new();
        apis.insert("mc", vec!["2.0", "2.0-alpha"]);
        apis.insert("mc-prompt", vec!["2.0"]);
        apis.insert("mc-env", vec!["2.0"]);
        apis.insert("mc-flows", vec!["2.0"]);
        apis.insert("mc-repo", vec!["2.0"]);
        apis.insert("mc-steps", vec!["2.0", "2.0-alpha"]);
        apis.insert("mc-container", vec!["1.0", "1.0-beta"]);
        apis.insert("mc-template", vec!["2.0", "2.0-alpha"]);
        
        match apis.get(api.as_str()) {
            Some(x) => {
                let mut cont = false;
                for i in x {
                    if i == &version.as_str() {
                        cont = true;
                    }
                }
                cont
            },
            None => {false}
        }
        
        
    }
}