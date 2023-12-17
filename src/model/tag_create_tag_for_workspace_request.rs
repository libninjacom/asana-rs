
use serde::{Serialize, Deserialize};
use super::TagBase;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagCreateTagForWorkspaceRequest {
    #[serde(flatten)]
    pub tag_base: TagBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<Vec<String>>,
}
impl std::fmt::Display for TagCreateTagForWorkspaceRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for TagCreateTagForWorkspaceRequest {
    type Target = TagBase;
    fn deref(&self) -> &Self::Target {
        &self.tag_base
    }
}
impl std::ops::DerefMut for TagCreateTagForWorkspaceRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tag_base
    }
}