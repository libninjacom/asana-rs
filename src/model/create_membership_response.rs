use serde::{Serialize, Deserialize};
use super::MembershipResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateMembershipResponse {
    pub data: MembershipResponse,
}
impl std::fmt::Display for CreateMembershipResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
