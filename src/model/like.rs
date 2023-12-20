use serde::{Serialize, Deserialize};
use super::UserCompact;
///An object to represent a user's like.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Like {
    ///Globally unique identifier of the object, as a string.
    pub gid: String,
    pub user: UserCompact,
}
impl std::fmt::Display for Like {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}