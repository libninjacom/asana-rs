
use serde::{Serialize, Deserialize};
use super::CustomFieldSettingResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddCustomFieldSettingForPortfolioResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CustomFieldSettingResponse>,
}
impl std::fmt::Display for AddCustomFieldSettingForPortfolioResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}