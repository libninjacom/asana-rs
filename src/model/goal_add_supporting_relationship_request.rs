
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GoalAddSupportingRelationshipRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contribution_weight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_before: Option<String>,
    pub supporting_resource: String,
}
impl std::fmt::Display for GoalAddSupportingRelationshipRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}