
use serde::{Serialize, Deserialize};
use super::GoalRelationshipResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateGoalRelationshipResponse {
    pub data: GoalRelationshipResponse,
}
impl std::fmt::Display for UpdateGoalRelationshipResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}