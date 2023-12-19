
use serde::{Serialize, Deserialize};
use super::{AsanaResource, ProjectCompact, ProjectTemplateCompact};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JobCompact {
    ///A generic Asana Resource, containing a globally unique identifier.
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_project: Option<ProjectCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_project_template: Option<ProjectTemplateCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_task: Option<serde_json::Value>,
    ///The subtype of this resource. Different subtypes retain many of the same fields and behavior, but may render differently in Asana or represent resources with different semantic meaning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_subtype: Option<String>,
    ///The current status of this job. The value is one of: `not_started`, `in_progress`, `succeeded`, or `failed`.
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