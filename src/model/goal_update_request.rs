
use serde::{Serialize, Deserialize};
use super::GoalRequestBase;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalUpdateRequest {
    #[serde(flatten)]
    pub goal_request_base: GoalRequestBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl std::fmt::Display for GoalUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for GoalUpdateRequest {
    type Target = GoalRequestBase;
    fn deref(&self) -> &Self::Target {
        &self.goal_request_base
    }
}
impl std::ops::DerefMut for GoalUpdateRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.goal_request_base
    }
}