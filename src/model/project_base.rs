
use serde::{Serialize, Deserialize};
use super::{CustomFieldSettingResponse, ProjectCompact, UserCompact};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectBase {
    #[serde(flatten)]
    pub project_compact: ProjectCompact,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_status: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_status_update: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_settings: Option<Vec<CustomFieldSettingResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_access_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_view: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_on: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<UserCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_access_level_for_customization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_access_level_for_sharing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_on: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for ProjectBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ProjectBase {
    type Target = ProjectCompact;
    fn deref(&self) -> &Self::Target {
        &self.project_compact
    }
}
impl std::ops::DerefMut for ProjectBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.project_compact
    }
}