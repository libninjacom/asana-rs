
use serde::{Serialize, Deserialize};
use super::UserCompact;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct GetUsersForWorkspaceResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<UserCompact>>,
}
impl std::fmt::Display for GetUsersForWorkspaceResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}