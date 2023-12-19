
use serde::{Serialize, Deserialize};
use super::TaskTemplateResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTaskTemplateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TaskTemplateResponse>,
}
impl std::fmt::Display for GetTaskTemplateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}