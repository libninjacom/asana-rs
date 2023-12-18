
use serde::{Serialize, Deserialize};
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct GoalRemoveSupportingRelationshipRequest {
    ///The gid of the supporting resource to remove from the parent goal. Must be the gid of a goal, project, task, or portfolio.
    pub supporting_resource: String,
}
impl std::fmt::Display for GoalRemoveSupportingRelationshipRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}