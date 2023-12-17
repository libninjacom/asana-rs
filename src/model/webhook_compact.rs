
use serde::{Serialize, Deserialize};
use super::{AsanaNamedResource, AsanaResource};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookCompact {
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<AsanaNamedResource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl std::fmt::Display for WebhookCompact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for WebhookCompact {
    type Target = AsanaResource;
    fn deref(&self) -> &Self::Target {
        &self.asana_resource
    }
}
impl std::ops::DerefMut for WebhookCompact {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asana_resource
    }
}