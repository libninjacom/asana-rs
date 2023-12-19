
use serde::{Serialize, Deserialize};
///A set of dependent tasks.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyDependentsRequest {
    ///An array of task gids that are dependents of the given task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependents: Option<Vec<String>>,
}
impl std::fmt::Display for ModifyDependentsRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}