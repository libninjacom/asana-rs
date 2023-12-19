use serde::{Serialize, Deserialize};
///A user identification object for specification with the addUser/removeUser endpoints.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceAddUserRequest {
    ///A string identifying a user. This can either be the string "me", an email, or the gid of a user.
    pub user: String,
}
impl std::fmt::Display for WorkspaceAddUserRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
