
use serde::{Serialize, Deserialize};
use super::AsanaResource;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GoalBase {
    ///A generic Asana Resource, containing a globally unique identifier.
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    ///The localized day on which this goal is due. This takes a date with format `YYYY-MM-DD`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_on: Option<String>,
    ///The notes of the goal with formatting as HTML.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_notes: Option<String>,
    ///*Conditional*. This property is only present when the `workspace` provided is an organization. Whether the goal belongs to the `workspace` (and is listed as part of the workspace’s goals) or not. If it isn’t a workspace-level goal, it is a team-level goal, and is associated with the goal’s team.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_workspace_level: Option<bool>,
    ///True if the goal is liked by the authorized user, false if not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liked: Option<bool>,
    ///The name of the goal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Free-form textual information associated with the goal (i.e. its description).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    ///The day on which work for this goal begins, or null if the goal has no start date. This takes a date with `YYYY-MM-DD` format, and cannot be set unless there is an accompanying due date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_on: Option<String>,
}
impl std::fmt::Display for GoalBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for GoalBase {
    type Target = AsanaResource;
    fn deref(&self) -> &Self::Target {
        &self.asana_resource
    }
}
impl std::ops::DerefMut for GoalBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asana_resource
    }
}