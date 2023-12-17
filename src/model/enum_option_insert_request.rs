
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnumOptionInsertRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_enum_option: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_enum_option: Option<String>,
    pub enum_option: String,
}
impl std::fmt::Display for EnumOptionInsertRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}