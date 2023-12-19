
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyDependenciesRequest {
    ///An array of task gids that a task depends on.
    pub dependencies: Vec<String>,
}
impl std::fmt::Display for ModifyDependenciesRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}