
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskAddProjectRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_before: Option<String>,
    pub project: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
}
impl std::fmt::Display for TaskAddProjectRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}