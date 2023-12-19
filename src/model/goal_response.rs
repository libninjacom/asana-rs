
use serde::{Serialize, Deserialize};
use super::{GoalBase, Like, UserCompact};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GoalResponse {
    #[serde(flatten)]
    pub goal_base: GoalBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_status_update: Option<serde_json::Value>,
    ///Array of users who are members of this goal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<Vec<UserCompact>>,
    ///Array of likes for users who have liked this goal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes: Option<Vec<Like>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<serde_json::Value>,
    ///The number of users who have liked this goal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_likes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<serde_json::Value>,
    /**The current status of this goal. When the goal is open, its status can be `green`, `yellow`, and `red` to reflect "On Track", "At Risk", and "Off Track", respectively. When the goal is closed, the value can be `missed`, `achieved`, `partial`, or `dropped`.
*Note* you can only write to this property if `metric` is set.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<serde_json::Value>,
}
impl std::fmt::Display for GoalResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for GoalResponse {
    type Target = GoalBase;
    fn deref(&self) -> &Self::Target {
        &self.goal_base
    }
}
impl std::ops::DerefMut for GoalResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.goal_base
    }
}