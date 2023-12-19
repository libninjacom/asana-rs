use serde::{Serialize, Deserialize};
use super::ProjectBase;
impl ProjectRequest {
    pub fn new(name: impl Into<String>) -> Self {
        use crate::model::ProjectCompact;
        Self {
            project_base: ProjectBase {
                project_compact: ProjectCompact {
                    name: Some(name.into()),
                    ..ProjectCompact::default()
                },
                ..ProjectBase::default()
            },
            ..Self::default()
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectRequest {
    #[serde(flatten)]
    pub project_base: ProjectBase,
    ///An object where each key is the GID of a custom field and its corresponding value is either an enum GID, string, number, or object (depending on the custom field type). See the [custom fields guide](/docs/custom-fields-guide) for details on creating and updating custom field values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    ///*Create-only*. Comma separated string of users. Followers are a subset of members who have opted in to receive "tasks added" notifications for a project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<String>,
    ///The current owner of the project, may be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    ///The team that this project is shared with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
    ///The `gid` of a workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
}
impl std::fmt::Display for ProjectRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ProjectRequest {
    type Target = ProjectBase;
    fn deref(&self) -> &Self::Target {
        &self.project_base
    }
}
impl std::ops::DerefMut for ProjectRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.project_base
    }
}