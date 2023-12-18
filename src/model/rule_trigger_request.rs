
use serde::{Serialize, Deserialize};
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuleTriggerRequest {
    ///The dynamic keys and values of the request. These fields are intended to be used in the action for the rule associated with this trigger.
    pub action_data: serde_json::Value,
    ///The ID of the resource. For the duration of the beta, this resource is always a task, and this task must exist in the project in which the rule is created.
    pub resource: String,
}
impl std::fmt::Display for RuleTriggerRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}