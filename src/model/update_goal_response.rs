use serde::{Serialize, Deserialize};
use super::GoalResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateGoalResponse {
    pub data: GoalResponse,
}
impl std::fmt::Display for UpdateGoalResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}