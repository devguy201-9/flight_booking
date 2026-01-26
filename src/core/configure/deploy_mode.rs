use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeployMode {
    Mono,
    Micro,
}

impl DeployMode {
    pub fn is_micro(&self) -> bool {
        matches!(self, DeployMode::Micro)
    }
}