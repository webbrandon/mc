pub mod details;
pub mod env_file;
pub mod env_prompt;
pub mod flows;
pub mod repository;
pub mod specs;
pub mod steps;

pub use details::Details;
pub use specs::Specs;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MasterOfCeremonyModel {
    pub api: String,
    pub version: String,
    pub metadata: Option<Details>,
    pub specs: Specs,
}
