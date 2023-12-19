
use serde::{Serialize, Deserialize};
use super::TagBase;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TagRequest {
    #[serde(flatten)]
    pub tag_base: TagBase,
    ///An array of strings identifying users. These can either be the string "me", an email, or the gid of a user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<Vec<String>>,
    ///Gid of an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
}
impl std::fmt::Display for TagRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for TagRequest {
    type Target = TagBase;
    fn deref(&self) -> &Self::Target {
        &self.tag_base
    }
}
impl std::ops::DerefMut for TagRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tag_base
    }
}