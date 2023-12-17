
use serde::{Serialize, Deserialize};
use super::CustomFieldResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateCustomFieldResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CustomFieldResponse>,
}
impl std::fmt::Display for CreateCustomFieldResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}