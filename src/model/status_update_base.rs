use serde::{Serialize, Deserialize};
use super::StatusUpdateCompact;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatusUpdateBase {
    #[serde(flatten)]
    pub status_update_compact: StatusUpdateCompact,
    ///[Opt In](/docs/inputoutput-options). The text content of the status update with formatting as HTML.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_text: Option<String>,
    ///The type associated with the status update. This represents the current state of the object this object is on.
    pub status_type: String,
    ///The text content of the status update.
    pub text: String,
}
impl std::fmt::Display for StatusUpdateBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for StatusUpdateBase {
    type Target = StatusUpdateCompact;
    fn deref(&self) -> &Self::Target {
        &self.status_update_compact
    }
}
impl std::ops::DerefMut for StatusUpdateBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.status_update_compact
    }
}