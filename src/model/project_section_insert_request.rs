
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectSectionInsertRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_section: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_section: Option<String>,
    pub section: String,
}
impl std::fmt::Display for ProjectSectionInsertRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}