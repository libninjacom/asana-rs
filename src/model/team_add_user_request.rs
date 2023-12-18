
use serde::{Serialize, Deserialize};
use fake::Dummy;
///A user identification object for specification with the addUser/removeUser endpoints.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct TeamAddUserRequest {
    ///A string identifying a user. This can either be the string "me", an email, or the gid of a user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
impl std::fmt::Display for TeamAddUserRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}