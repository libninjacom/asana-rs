
use serde::{Serialize, Deserialize};
use super::EnumOptionBase;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumOptionRequest {
    #[serde(flatten)]
    pub enum_option_base: EnumOptionBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_before: Option<String>,
}
impl std::fmt::Display for EnumOptionRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for EnumOptionRequest {
    type Target = EnumOptionBase;
    fn deref(&self) -> &Self::Target {
        &self.enum_option_base
    }
}
impl std::ops::DerefMut for EnumOptionRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.enum_option_base
    }
}