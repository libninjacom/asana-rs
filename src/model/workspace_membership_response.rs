
use serde::{Serialize, Deserialize};
use super::{UserTaskListResponse, WorkspaceMembershipBase};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceMembershipResponse {
    #[serde(flatten)]
    pub workspace_membership_base: WorkspaceMembershipBase,
    ///The time at which this resource was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    ///Reflects if this user still a member of the workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    ///Reflects if this user is an admin of the workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
    ///Reflects if this user is a guest of the workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_guest: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_task_list: Option<UserTaskListResponse>,
    ///Contains keys `start_on` and `end_on` for the vacation dates for the user in this workspace. If `start_on` is null, the entire `vacation_dates` object will be null. If `end_on` is before today, the entire `vacation_dates` object will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vacation_dates: Option<serde_json::Value>,
}
impl std::fmt::Display for WorkspaceMembershipResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for WorkspaceMembershipResponse {
    type Target = WorkspaceMembershipBase;
    fn deref(&self) -> &Self::Target {
        &self.workspace_membership_base
    }
}
impl std::ops::DerefMut for WorkspaceMembershipResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.workspace_membership_base
    }
}