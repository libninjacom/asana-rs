
use serde::{Serialize, Deserialize};
use super::GoalRelationshipCompact;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct GoalRelationshipBase {
    #[serde(flatten)]
    pub goal_relationship_compact: GoalRelationshipCompact,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_goal: Option<serde_json::Value>,
}
impl std::fmt::Display for GoalRelationshipBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for GoalRelationshipBase {
    type Target = GoalRelationshipCompact;
    fn deref(&self) -> &Self::Target {
        &self.goal_relationship_compact
    }
}
impl std::ops::DerefMut for GoalRelationshipBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.goal_relationship_compact
    }
}