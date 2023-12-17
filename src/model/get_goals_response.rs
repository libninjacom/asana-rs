
use serde::{Serialize, Deserialize};
use super::{GoalCompact, NextPage};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetGoalsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GoalCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<NextPage>,
}
impl std::fmt::Display for GetGoalsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}