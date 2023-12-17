
use serde::{Serialize, Deserialize};
use super::ProjectBase;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRequest {
    #[serde(flatten)]
    pub project_base: ProjectBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
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