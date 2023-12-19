use serde::{Serialize, Deserialize};
use super::ProjectBase;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectUpdateRequest {
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
}
impl std::fmt::Display for ProjectUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ProjectUpdateRequest {
    type Target = ProjectBase;
    fn deref(&self) -> &Self::Target {
        &self.project_base
    }
}
impl std::ops::DerefMut for ProjectUpdateRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.project_base
    }
}
