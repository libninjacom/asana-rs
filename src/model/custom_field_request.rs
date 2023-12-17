
use serde::{Serialize, Deserialize};
use super::CustomFieldBase;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldRequest {
    #[serde(flatten)]
    pub custom_field_base: CustomFieldBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_by_app: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_value: Option<Vec<String>>,
    pub workspace: String,
}
impl std::fmt::Display for CustomFieldRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for CustomFieldRequest {
    type Target = CustomFieldBase;
    fn deref(&self) -> &Self::Target {
        &self.custom_field_base
    }
}
impl std::ops::DerefMut for CustomFieldRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.custom_field_base
    }
}