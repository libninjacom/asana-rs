use serde::{Serialize, Deserialize};
use super::ProjectBriefCompact;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectBriefBase {
    #[serde(flatten)]
    pub project_brief_compact: ProjectBriefCompact,
    ///HTML formatted text for the project brief.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_text: Option<String>,
    ///The title of the project brief.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl std::fmt::Display for ProjectBriefBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ProjectBriefBase {
    type Target = ProjectBriefCompact;
    fn deref(&self) -> &Self::Target {
        &self.project_brief_compact
    }
}
impl std::ops::DerefMut for ProjectBriefBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.project_brief_compact
    }
}