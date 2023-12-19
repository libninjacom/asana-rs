use serde::{Serialize, Deserialize};
use super::{CustomFieldCompact, ProjectBase, UserCompact};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectResponse {
    #[serde(flatten)]
    pub project_base: ProjectBase,
    ///True if the project is currently marked complete, false if not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    ///The time at which this project was completed, or null if the project is not completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_by: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_from_template: Option<serde_json::Value>,
    ///Array of Custom Fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldCompact>>,
    ///Array of users following this project. Followers are a subset of members who have opted in to receive "tasks added" notifications for a project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<Vec<UserCompact>>,
    ///The icon for a project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    ///The current owner of the project, may be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<serde_json::Value>,
    ///A url that points directly to the object within Asana.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permalink_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_brief: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<serde_json::Value>,
}
impl std::fmt::Display for ProjectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ProjectResponse {
    type Target = ProjectBase;
    fn deref(&self) -> &Self::Target {
        &self.project_base
    }
}
impl std::ops::DerefMut for ProjectResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.project_base
    }
}
