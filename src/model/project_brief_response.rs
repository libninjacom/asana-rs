
use serde::{Serialize, Deserialize};
use super::ProjectBriefBase;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct ProjectBriefResponse {
    #[serde(flatten)]
    pub project_brief_base: ProjectBriefBase,
    ///A url that points directly to the object within Asana.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permalink_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<serde_json::Value>,
    ///[Opt In](/docs/inputoutput-options). The plain text of the project brief.
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