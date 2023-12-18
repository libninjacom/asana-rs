
use serde::{Serialize, Deserialize};
use super::GoalCompact;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct GetParentGoalsForGoalResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GoalCompact>>,
}
impl std::fmt::Display for GetParentGoalsForGoalResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}