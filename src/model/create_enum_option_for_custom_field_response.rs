
use serde::{Serialize, Deserialize};
use super::EnumOption;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateEnumOptionForCustomFieldResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<EnumOption>,
}
impl std::fmt::Display for CreateEnumOptionForCustomFieldResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}