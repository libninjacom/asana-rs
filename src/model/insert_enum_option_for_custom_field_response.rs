
use serde::{Serialize, Deserialize};
use super::EnumOption;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct InsertEnumOptionForCustomFieldResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<EnumOption>,
}
impl std::fmt::Display for InsertEnumOptionForCustomFieldResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}