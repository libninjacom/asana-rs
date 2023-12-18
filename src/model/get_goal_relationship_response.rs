
use serde::{Serialize, Deserialize};
use super::GoalRelationshipResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetGoalRelationshipResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GoalRelationshipResponse>,
}
impl std::fmt::Display for GetGoalRelationshipResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}