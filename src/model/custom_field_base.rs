use serde::{Serialize, Deserialize};
use super::{CustomFieldCompact, EnumOption};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomFieldBase {
    #[serde(flatten)]
    pub custom_field_compact: CustomFieldCompact,
    ///*Conditional*. A unique identifier to associate this field with the template source of truth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asana_created_field: Option<serde_json::Value>,
    ///ISO 4217 currency code to format this custom field. This will be null if the `format` is not `currency`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    ///This is the string that appears next to the custom field value. This will be null if the `format` is not `custom`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_label: Option<String>,
    ///Only relevant for custom fields with `custom` format. This depicts where to place the custom label. This will be null if the `format` is not `custom`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_label_position: Option<serde_json::Value>,
    ///[Opt In](/docs/inputoutput-options). The description of the custom field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///*Conditional*. Only relevant for custom fields of type `enum`. This array specifies the possible values which an `enum` custom field can adopt. To modify the enum options, refer to [working with enum options](/reference/createenumoptionforcustomfield).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_options: Option<Vec<EnumOption>>,
    ///The format of this custom field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    ///*Conditional*. This flag describes whether a follower of a task with this field should receive inbox notifications from changes to this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_notifications_enabled: Option<bool>,
    ///This flag describes whether this custom field is available to every container in the workspace. Before project-specific custom fields, this field was always true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_global_to_workspace: Option<bool>,
    /**Only relevant for custom fields of type ‘Number’. This field dictates the number of places after the decimal to round to, i.e. 0 is integer values, 1 rounds to the nearest tenth, and so on. Must be between 0 and 6, inclusive.
For percentage format, this may be unintuitive, as a value of 0.25 has a precision of 0, while a value of 0.251 has a precision of 1. This is due to 0.25 being displayed as 25%.
The identifier format will always have a precision of 0.*/
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
