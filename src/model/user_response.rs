
use serde::{Serialize, Deserialize};
use super::{UserBaseResponse, WorkspaceCompact};
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserResponse {
    #[serde(flatten)]
    pub user_base_response: UserBaseResponse,
    /**Workspaces and organizations this user may access.
Note\: The API will only return workspaces and organizations that also contain the authenticated user.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<Vec<WorkspaceCompact>>,
}
impl std::fmt::Display for UserResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for UserResponse {
    type Target = UserBaseResponse;
    fn deref(&self) -> &Self::Target {
        &self.user_base_response
    }
}
impl std::ops::DerefMut for UserResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.user_base_response
    }
}