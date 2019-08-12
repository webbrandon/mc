extern crate serde_yaml;
use crate::models::MasterOfCeremonyModel;

/// The ModelHandler will intake a file as a String and populate the model fields.  Files currently
/// supported as string only include YAML at the moment but will soon handle JSON.
#[derive(Debug, Default, Clone)]
pub struct MasterOfCeremonyModelHandler {}

impl MasterOfCeremonyModelHandler {
    pub fn from_file(yaml_file: String) -> MasterOfCeremonyModel {
        // Validate the YAML file structure.  Exit on invalid configuration, this is serde default.
        // Manage deserialization error to not exit and continue with default MasterOfCeremonyModel.
        let model: MasterOfCeremonyModel = serde_yaml::from_str(&yaml_file).unwrap();

        // Validate the api and version. Continue to run default configuration in case of cmd args.
        let is_valid_api_ver = Self {}.valid_api_ver(&model);
        match is_valid_api_ver {
            true => {}
            false => {
                println!("Unrecognized Master Of Ceremony file format!");
            }
        }

        model
    }

    /// Run validation against the api type and its version.
    pub fn valid_api_ver(&mut self, model: &MasterOfCeremonyModel) -> bool {
        match (
            self.valid_api_type(&model.api),
            self.valid_version(&model.api, &model.version),
        ) {
            (true, true) => true,
            (false, true) => {
                println!("Invalid Api");
                false
            }
            (true, false) => {
                println!("Invalid Version");
                false
            }
            (false, false) => {
                println!("Invalid Api Version");
                false
            }
        }
    }

    /// Validate the api types.
    fn valid_api_type(&mut self, api: &str) -> bool {
        if api == "mc" {
            true
        } else {
            false
        }
    }

    /// Validate all api versions.
    fn valid_version(&mut self, api: &str, version: &str) -> bool {
        // Will need to comeback and make a hashmap for various api's and their versions.
        // Consider using true subversion check (https://crates.io/crates/semver)
        if api == "mc" && version == "v1.0" {
            true
        } else {
            false
        }
    }
}
