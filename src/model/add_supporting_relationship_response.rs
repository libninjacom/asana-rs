
use serde::{Serialize, Deserialize};
use super::GoalRelationshipResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddSupportingRelationshipResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GoalRelationshipResponse>,
}
impl std::fmt::Display for AddSupportingRelationshipResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}