
use serde::{Serialize, Deserialize};
use super::AttachmentBase;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentResponse {
    #[serde(flatten)]
    pub attachment_base: AttachmentBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_to_app: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_url: Option<String>,
}
impl std::fmt::Display for AttachmentResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for AttachmentResponse {
    type Target = AttachmentBase;
    fn deref(&self) -> &Self::Target {
        &self.attachment_base
    }
}
impl std::ops::DerefMut for AttachmentResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attachment_base
    }
}