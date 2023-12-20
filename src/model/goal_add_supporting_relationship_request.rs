use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GoalAddSupportingRelationshipRequest {
    ///The weight that the supporting resource's progress will contribute to the supported goal's progress. This can only be 0 or 1.
    pub contribution_weight: f64,
    ///An id of a subgoal of this parent goal. The new subgoal will be added after the one specified here. `insert_before` and `insert_after` parameters cannot both be specified. Currently only supported when adding a subgoal.
    pub insert_after: String,
    ///An id of a subgoal of this parent goal. The new subgoal will be added before the one specified here. `insert_before` and `insert_after` parameters cannot both be specified. Currently only supported when adding a subgoal.
    pub insert_before: String,
    ///The gid of the supporting resource to add to the parent goal. Must be the gid of a goal, project, task, or portfolio.
    pub supporting_resource: String,
}
impl std::fmt::Display for GoalAddSupportingRelationshipRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}