use serde::{Serialize, Deserialize};
use super::{DateVariableCompact, ProjectTemplateCompact, TeamCompact, TemplateRole};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectTemplateBase {
    #[serde(flatten)]
    pub project_template_compact: ProjectTemplateCompact,
    ///Color of the project template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<serde_json::Value>,
    ///Free-form textual information associated with the project template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The description of the project template with formatting as HTML.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_description: Option<String>,
    ///The current owner of the project template, may be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<serde_json::Value>,
    ///True if the project template is public to its team.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    ///Array of date variables in this project template. Calendar dates must be provided for these variables when instantiating a project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_dates: Option<Vec<DateVariableCompact>>,
    ///Array of template roles in this project template. User Ids can be provided for these variables when instantiating a project to assign template tasks to the user.
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