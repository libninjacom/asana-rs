
use serde::{Serialize, Deserialize};
use super::WebhookCompact;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct WebhookResponse {
    #[serde(flatten)]
    pub webhook_compact: WebhookCompact,
    ///The time at which this resource was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    ///Whitelist of filters to apply to events from this webhook. If a webhook event passes any of the filters the event will be delivered; otherwise no event will be sent to the receiving server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<serde_json::Value>>,
    ///The timestamp when the webhook last received an error when sending an event to the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_at: Option<chrono::DateTime<chrono::Utc>>,
    ///The contents of the last error response sent to the webhook when attempting to deliver events to the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_content: Option<String>,
    ///The timestamp when the webhook last successfully sent an event to the target.
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