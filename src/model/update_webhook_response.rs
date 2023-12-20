use serde::{Serialize, Deserialize};
use super::WebhookResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateWebhookResponse {
    pub data: WebhookResponse,
}
impl std::fmt::Display for UpdateWebhookResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}