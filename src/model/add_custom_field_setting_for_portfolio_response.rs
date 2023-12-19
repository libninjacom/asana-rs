
use serde::{Serialize, Deserialize};
use super::CustomFieldSettingResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddCustomFieldSettingForPortfolioResponse {
    pub data: CustomFieldSettingResponse,
}
impl std::fmt::Display for AddCustomFieldSettingForPortfolioResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}