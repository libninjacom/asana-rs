
use serde::{Serialize, Deserialize};
use super::TaskTemplateCompact;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTaskTemplatesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<TaskTemplateCompact>>,
}
impl std::fmt::Display for GetTaskTemplatesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}