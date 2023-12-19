use serde::{Serialize, Deserialize};
use super::{CustomFieldBase, UserCompact};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomFieldResponse {
    #[serde(flatten)]
    pub custom_field_base: CustomFieldBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<serde_json::Value>,
    ///*Conditional*. This flag describes whether a custom field is a formula custom field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_formula_field: Option<bool>,
    ///*Conditional*. This flag describes whether a custom field is read only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_value_read_only: Option<bool>,
    ///*Conditional*. Only relevant for custom fields of type `people`. This array of [compact user](/reference/users) objects reflects the values of a `people` custom field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_value: Option<Vec<UserCompact>>,
}
impl std::fmt::Display for CustomFieldResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for CustomFieldResponse {
    type Target = CustomFieldBase;
    fn deref(&self) -> &Self::Target {
        &self.custom_field_base
    }
}
impl std::ops::DerefMut for CustomFieldResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.custom_field_base
    }
}
