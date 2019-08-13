use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct ApiValidateHandler {}

impl ApiValidateHandler {
    /// Run validation against the api type and its version.
    pub fn valid_api_ver(api: String, version: String) -> bool {
        let mut apis = HashMap::new();
        apis.insert("mc", vec!["1.0", "1.0-rc"]);
        apis.insert("mc-prompt", vec!["1.0", "1.0-rc"]);
        apis.insert("mc-env", vec!["1.0", "1.0-rc"]);
        apis.insert("mc-flows", vec!["1.0", "1.0-rc"]);
        apis.insert("mc-repo", vec!["1.0", "1.0-rc"]);
        
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