
use serde::{Serialize, Deserialize};
use super::UserResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetUserResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<UserResponse>,
}
impl std::fmt::Display for GetUserResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}