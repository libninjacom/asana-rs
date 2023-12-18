
use serde::{Serialize, Deserialize};
use super::UserBaseResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddUserForWorkspaceResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<UserBaseResponse>,
}
impl std::fmt::Display for AddUserForWorkspaceResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}