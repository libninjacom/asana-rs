
use serde::{Serialize, Deserialize};
use super::EnumOption;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateEnumOptionResponse {
    pub data: EnumOption,
}
impl std::fmt::Display for UpdateEnumOptionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}