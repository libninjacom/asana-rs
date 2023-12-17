
use serde::{Serialize, Deserialize};
use super::ProjectBriefBase;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectBriefResponse {
    #[serde(flatten)]
    pub project_brief_base: ProjectBriefBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permalink_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
impl std::fmt::Display for ProjectBriefResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ProjectBriefResponse {
    type Target = ProjectBriefBase;
    fn deref(&self) -> &Self::Target {
        &self.project_brief_base
    }
}
impl std::ops::DerefMut for ProjectBriefResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.project_brief_base
    }
}