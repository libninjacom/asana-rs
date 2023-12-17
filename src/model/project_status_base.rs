
use serde::{Serialize, Deserialize};
use super::ProjectStatusCompact;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStatusBase {
    #[serde(flatten)]
    pub project_status_compact: ProjectStatusCompact,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
impl std::fmt::Display for ProjectStatusBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ProjectStatusBase {
    type Target = ProjectStatusCompact;
    fn deref(&self) -> &Self::Target {
        &self.project_status_compact
    }
}
impl std::ops::DerefMut for ProjectStatusBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.project_status_compact
    }
}