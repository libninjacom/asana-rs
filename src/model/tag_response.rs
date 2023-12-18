
use serde::{Serialize, Deserialize};
use super::{TagBase, UserCompact, WorkspaceCompact};
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct TagResponse {
    #[serde(flatten)]
    pub tag_base: TagBase,
    ///The time at which this resource was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    ///Array of users following this tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<Vec<UserCompact>>,
    ///A url that points directly to the object within Asana.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permalink_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<WorkspaceCompact>,
}
impl std::fmt::Display for TagResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for TagResponse {
    type Target = TagBase;
    fn deref(&self) -> &Self::Target {
        &self.tag_base
    }
}
impl std::ops::DerefMut for TagResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tag_base
    }
}