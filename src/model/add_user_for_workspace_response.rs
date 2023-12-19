use serde::{Serialize, Deserialize};
use super::UserBaseResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddUserForWorkspaceResponse {
    pub data: UserBaseResponse,
}
impl std::fmt::Display for AddUserForWorkspaceResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
