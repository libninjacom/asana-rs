
use serde::{Serialize, Deserialize};
use super::{DateVariableCompact, ProjectTemplateCompact, TeamCompact, TemplateRole};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTemplateBase {
    #[serde(flatten)]
    pub project_template_compact: ProjectTemplateCompact,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_dates: Option<Vec<DateVariableCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_roles: Option<Vec<TemplateRole>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<TeamCompact>,
}
impl std::fmt::Display for ProjectTemplateBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ProjectTemplateBase {
    type Target = ProjectTemplateCompact;
    fn deref(&self) -> &Self::Target {
        &self.project_template_compact
    }
}
impl std::ops::DerefMut for ProjectTemplateBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.project_template_compact
    }
}