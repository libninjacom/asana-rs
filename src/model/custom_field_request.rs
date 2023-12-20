use serde::{Serialize, Deserialize};
use super::CustomFieldBase;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomFieldRequest {
    #[serde(flatten)]
    pub custom_field_base: CustomFieldBase,
    ///*Allow-listed*. Instructs the API that this Custom Field is app-owned. This parameter is allow-listed to specific apps at this point in time. For apps that are not allow-listed, providing this parameter will result in a `403 Forbidden`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_by_app: Option<bool>,
    ///*Conditional*. Only relevant for custom fields of type `people`. This array of user GIDs reflects the users to be written to a `people` custom field. Note that *write* operations will replace existing users (if any) in the custom field with the users specified in this array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_value: Option<Vec<String>>,
    ///*Create-Only* The workspace to create a custom field in.
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