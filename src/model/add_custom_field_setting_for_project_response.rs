use serde::{Serialize, Deserialize};
use super::CustomFieldSettingResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddCustomFieldSettingForProjectResponse {
    pub data: CustomFieldSettingResponse,
}
impl std::fmt::Display for AddCustomFieldSettingForProjectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
