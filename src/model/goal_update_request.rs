use serde::{Serialize, Deserialize};
use super::GoalRequestBase;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GoalUpdateRequest {
    #[serde(flatten)]
    pub goal_request_base: GoalRequestBase,
    /**The current status of this goal. When the goal is open, its status can be `green`, `yellow`, and `red` to reflect "On Track", "At Risk", and "Off Track", respectively. When the goal is closed, the value can be `missed`, `achieved`, `partial`, or `dropped`.
*Note* you can only write to this property if `metric` is set.*/
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