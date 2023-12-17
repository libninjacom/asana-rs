
use serde::{Serialize, Deserialize};
use super::{AsanaResource, ProjectCompact, ProjectTemplateCompact};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobCompact {
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_project: Option<ProjectCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_project_template: Option<ProjectTemplateCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_task: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl std::fmt::Display for JobCompact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for JobCompact {
    type Target = AsanaResource;
    fn deref(&self) -> &Self::Target {
        &self.asana_resource
    }
}
impl std::ops::DerefMut for JobCompact {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asana_resource
    }
}