
use serde::{Serialize, Deserialize};
use super::{CustomFieldCompact, ProjectBase, UserCompact};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectResponse {
    #[serde(flatten)]
    pub project_base: ProjectBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_by: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_from_template: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<Vec<UserCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<serde_json::Value>,
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