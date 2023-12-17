
use serde::{Serialize, Deserialize};
use super::WebhookCompact;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookResponse {
    #[serde(flatten)]
    pub webhook_compact: WebhookCompact,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_success_at: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for WebhookResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for WebhookResponse {
    type Target = WebhookCompact;
    fn deref(&self) -> &Self::Target {
        &self.webhook_compact
    }
}
impl std::ops::DerefMut for WebhookResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.webhook_compact
    }
}