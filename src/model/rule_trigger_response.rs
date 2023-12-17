
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuleTriggerResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl std::fmt::Display for RuleTriggerResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}