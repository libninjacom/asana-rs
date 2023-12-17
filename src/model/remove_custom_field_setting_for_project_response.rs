
use serde::{Serialize, Deserialize};
use super::EmptyResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoveCustomFieldSettingForProjectResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<EmptyResponse>,
}
impl std::fmt::Display for RemoveCustomFieldSettingForProjectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}