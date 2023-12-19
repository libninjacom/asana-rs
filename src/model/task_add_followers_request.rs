use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskAddFollowersRequest {
    ///An array of strings identifying users. These can either be the string "me", an email, or the gid of a user.
    pub followers: Vec<String>,
}
impl std::fmt::Display for TaskAddFollowersRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
