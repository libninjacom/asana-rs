
use serde::{Serialize, Deserialize};
use super::{NextPage, UserCompact};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetUsersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<UserCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<NextPage>,
}
impl std::fmt::Display for GetUsersResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}