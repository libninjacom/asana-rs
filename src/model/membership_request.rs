use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MembershipRequest {
    ///*Optional*. Denotes if a member is active. Applies to all memberships
    pub is_active: bool,
}
impl std::fmt::Display for MembershipRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
