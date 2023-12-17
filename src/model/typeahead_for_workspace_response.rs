
use serde::{Serialize, Deserialize};
use super::AsanaNamedResource;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TypeaheadForWorkspaceResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<AsanaNamedResource>>,
}
impl std::fmt::Display for TypeaheadForWorkspaceResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}