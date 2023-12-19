
use serde::{Serialize, Deserialize};
use super::AsanaResource;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GoalRelationshipCompact {
    ///A generic Asana Resource, containing a globally unique identifier.
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    ///The weight that the supporting resource's progress contributes to the supported goal's progress. This can only be 0 or 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contribution_weight: Option<f64>,
    ///The subtype of this resource. Different subtypes retain many of the same fields and behavior, but may render differently in Asana or represent resources with different semantic meaning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_resource: Option<serde_json::Value>,
}
impl std::fmt::Display for GoalRelationshipCompact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for GoalRelationshipCompact {
    type Target = AsanaResource;
    fn deref(&self) -> &Self::Target {
        &self.asana_resource
    }
}
impl std::ops::DerefMut for GoalRelationshipCompact {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asana_resource
    }
}