
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectSectionInsertRequest {
    ///Insert the given section immediately after the section specified by this parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_section: Option<String>,
    ///Insert the given section immediately before the section specified by this parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_section: Option<String>,
    ///The section to reorder.
    pub section: String,
}
impl std::fmt::Display for ProjectSectionInsertRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}