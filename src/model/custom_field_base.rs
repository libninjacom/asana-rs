
use serde::{Serialize, Deserialize};
use super::{CustomFieldCompact, EnumOption};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldBase {
    #[serde(flatten)]
    pub custom_field_compact: CustomFieldCompact,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asana_created_field: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_label_position: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_options: Option<Vec<EnumOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_notifications_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_global_to_workspace: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<i64>,
}
impl std::fmt::Display for CustomFieldBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for CustomFieldBase {
    type Target = CustomFieldCompact;
    fn deref(&self) -> &Self::Target {
        &self.custom_field_compact
    }
}
impl std::ops::DerefMut for CustomFieldBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.custom_field_compact
    }
}