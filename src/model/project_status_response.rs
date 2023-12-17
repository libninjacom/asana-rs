
use serde::{Serialize, Deserialize};
use super::{ProjectStatusBase, UserCompact};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStatusResponse {
    #[serde(flatten)]
    pub project_status_base: ProjectStatusBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<UserCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for ProjectStatusResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ProjectStatusResponse {
    type Target = ProjectStatusBase;
    fn deref(&self) -> &Self::Target {
        &self.project_status_base
    }
}
impl std::ops::DerefMut for ProjectStatusResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.project_status_base
    }
}