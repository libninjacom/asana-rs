use serde::{Serialize, Deserialize};
use super::RuleTriggerResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TriggerRuleResponse {
    pub data: RuleTriggerResponse,
}
impl std::fmt::Display for TriggerRuleResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
