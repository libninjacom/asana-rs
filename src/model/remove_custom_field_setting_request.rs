use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoveCustomFieldSettingRequest {
    ///The custom field to remove from this portfolio.
    pub custom_field: String,
}
impl std::fmt::Display for RemoveCustomFieldSettingRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
