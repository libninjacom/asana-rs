use serde::{Serialize, Deserialize};
use super::UserTaskListResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetUserTaskListForUserResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<UserTaskListResponse>,
}
impl std::fmt::Display for GetUserTaskListForUserResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
