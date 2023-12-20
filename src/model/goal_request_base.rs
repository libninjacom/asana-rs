use serde::{Serialize, Deserialize};
use super::GoalBase;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GoalRequestBase {
    #[serde(flatten)]
    pub goal_base: GoalBase,
    ///The `gid` of a user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    ///*Conditional*. This property is only present when the `workspace` provided is an organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
    ///The `gid` of a time period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<String>,
    ///The `gid` of a workspace.
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