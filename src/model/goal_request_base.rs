
use serde::{Serialize, Deserialize};
use super::GoalBase;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalRequestBase {
    #[serde(flatten)]
    pub goal_base: GoalBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
}
impl std::fmt::Display for GoalRequestBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for GoalRequestBase {
    type Target = GoalBase;
    fn deref(&self) -> &Self::Target {
        &self.goal_base
    }
}
impl std::ops::DerefMut for GoalRequestBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.goal_base
    }
}