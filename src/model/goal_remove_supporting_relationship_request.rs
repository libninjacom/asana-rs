
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GoalRemoveSupportingRelationshipRequest {
    pub supporting_resource: String,
}
impl std::fmt::Display for GoalRemoveSupportingRelationshipRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}