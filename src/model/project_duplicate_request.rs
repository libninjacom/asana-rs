
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectDuplicateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_dates: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
}
impl std::fmt::Display for ProjectDuplicateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}