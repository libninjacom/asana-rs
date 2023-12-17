
use serde::{Serialize, Deserialize};
use super::WebhookResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetWebhookResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<WebhookResponse>,
}
impl std::fmt::Display for GetWebhookResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}