
use serde::{Serialize, Deserialize};
use super::GoalResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct RemoveFollowersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GoalResponse>,
}
impl std::fmt::Display for RemoveFollowersResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}