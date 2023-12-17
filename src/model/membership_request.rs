
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MembershipRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}
impl std::fmt::Display for MembershipRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}