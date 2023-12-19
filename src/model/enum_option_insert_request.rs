
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnumOptionInsertRequest {
    ///An existing enum option within this custom field after which the new enum option should be inserted. Cannot be provided together with before_enum_option.
    pub after_enum_option: String,
    ///An existing enum option within this custom field before which the new enum option should be inserted. Cannot be provided together with after_enum_option.
    pub before_enum_option: String,
    ///The gid of the enum option to relocate.
    pub enum_option: String,
}
impl std::fmt::Display for EnumOptionInsertRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}