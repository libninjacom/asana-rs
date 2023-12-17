
use serde::{Serialize, Deserialize};
use super::AsanaResource;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusUpdateCompact {
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl std::fmt::Display for StatusUpdateCompact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for StatusUpdateCompact {
    type Target = AsanaResource;
    fn deref(&self) -> &Self::Target {
        &self.asana_resource
    }
}
impl std::ops::DerefMut for StatusUpdateCompact {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asana_resource
    }
}