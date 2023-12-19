use serde::{Serialize, Deserialize};
use super::{AsanaResource, EnumOption};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomFieldCompact {
    ///A generic Asana Resource, containing a globally unique identifier.
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    ///*Conditional*. Only relevant for custom fields of type `date`. This object reflects the chosen date (and optionally, time) value of a `date` custom field. If no date is selected, the value of `date_value` will be `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_value: Option<serde_json::Value>,
    ///A string representation for the value of the custom field. Integrations that don't require the underlying type should use this field to read values. Using this field will future-proof an app against new custom field types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_value: Option<String>,
    ///*Conditional*. Determines if the custom field is enabled or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    ///*Conditional*. Only relevant for custom fields of type `enum`. This array specifies the possible values which an `enum` custom field can adopt. To modify the enum options, refer to [working with enum options](/reference/createenumoptionforcustomfield).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_options: Option<Vec<EnumOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_value: Option<serde_json::Value>,
    ///*Conditional*. This flag describes whether a custom field is a formula custom field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_formula_field: Option<bool>,
    ///*Conditional*. Only relevant for custom fields of type `multi_enum`. This object is the chosen values of a `multi_enum` custom field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_enum_values: Option<Vec<EnumOption>>,
    ///The name of the custom field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///*Conditional*. This number is the value of a `number` custom field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_value: Option<f64>,
    ///The type of the custom field. Must be one of the given values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_subtype: Option<String>,
    ///*Conditional*. This string is the value of a `text` custom field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    ///*Deprecated: new integrations should prefer the resource_subtype field.* The type of the custom field. Must be one of the given values.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for CustomFieldCompact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for CustomFieldCompact {
    type Target = AsanaResource;
    fn deref(&self) -> &Self::Target {
        &self.asana_resource
    }
}
impl std::ops::DerefMut for CustomFieldCompact {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asana_resource
    }
}
