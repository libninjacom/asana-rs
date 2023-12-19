
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WebhookRequest {
    ///An array of WebhookFilter objects to specify a whitelist of filters to apply to events from this webhook. If a webhook event passes any of the filters the event will be delivered; otherwise no event will be sent to the receiving server.
    pub filters: Vec<serde_json::Value>,
    ///A resource ID to subscribe to. Many Asana resources are valid to create webhooks on, but higher-level resources require filters.
    pub resource: String,
    ///The URL to receive the HTTP POST. The full URL will be used to deliver events from this webhook (including parameters) which allows encoding of application-specific state when the webhook is created.
    pub target: String,
}
impl std::fmt::Display for WebhookRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}