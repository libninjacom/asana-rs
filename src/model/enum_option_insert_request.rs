
use serde::{Serialize, Deserialize};
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct EnumOptionInsertRequest {
    ///An existing enum option within this custom field after which the new enum option should be inserted. Cannot be provided together with before_enum_option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_enum_option: Option<String>,
    ///An existing enum option within this custom field before which the new enum option should be inserted. Cannot be provided together with after_enum_option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_enum_option: Option<String>,
    ///The gid of the enum option to relocate.
    pub enum_option: String,
}
impl std::fmt::Display for EnumOptionInsertRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}