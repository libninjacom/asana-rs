
use serde::{Serialize, Deserialize};
use super::ProjectResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddMembersForProjectResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ProjectResponse>,
}
impl std::fmt::Display for AddMembersForProjectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}