
use serde::{Serialize, Deserialize};
use super::TagCompact;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct TagBase {
    #[serde(flatten)]
    pub tag_compact: TagCompact,
    ///Color of the tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<serde_json::Value>,
    ///Free-form textual information associated with the tag (i.e. its description).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
impl std::fmt::Display for TagBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for TagBase {
    type Target = TagCompact;
    fn deref(&self) -> &Self::Target {
        &self.tag_compact
    }
}
impl std::ops::DerefMut for TagBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tag_compact
    }
}