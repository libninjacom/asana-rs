
use serde::{Serialize, Deserialize};
use super::GoalResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateGoalMetricResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GoalResponse>,
}
impl std::fmt::Display for UpdateGoalMetricResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}