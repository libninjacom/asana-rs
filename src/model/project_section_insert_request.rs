use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectSectionInsertRequest {
    ///Insert the given section immediately after the section specified by this parameter.
    pub after_section: String,
    ///Insert the given section immediately before the section specified by this parameter.
    pub before_section: String,
    ///The section to reorder.
    pub section: String,
}
impl std::fmt::Display for ProjectSectionInsertRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
