use serde::{Serialize, Deserialize};
use super::CustomFieldResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCustomFieldResponse {
    pub data: CustomFieldResponse,
}
impl std::fmt::Display for UpdateCustomFieldResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}