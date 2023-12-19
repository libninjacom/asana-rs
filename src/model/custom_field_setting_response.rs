
use serde::{Serialize, Deserialize};
use super::CustomFieldSettingBase;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomFieldSettingResponse {
    #[serde(flatten)]
    pub custom_field_setting_base: CustomFieldSettingBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field: Option<serde_json::Value>,
    ///`is_important` is used in the Asana web application to determine if this custom field is displayed in the list/grid view of a project or portfolio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_important: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<serde_json::Value>,
}
impl std::fmt::Display for CustomFieldSettingResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for CustomFieldSettingResponse {
    type Target = CustomFieldSettingBase;
    fn deref(&self) -> &Self::Target {
        &self.custom_field_setting_base
    }
}
impl std::ops::DerefMut for CustomFieldSettingResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.custom_field_setting_base
    }
}