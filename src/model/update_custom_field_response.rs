
use serde::{Serialize, Deserialize};
use super::CustomFieldResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCustomFieldResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CustomFieldResponse>,
}
impl std::fmt::Display for UpdateCustomFieldResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}