use super::Dockerization;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Containerization {
    pub docker: Option<Dockerization>,
}

impl Containerization {
    pub fn new() -> Self  {
        Default::default()
    }
}

