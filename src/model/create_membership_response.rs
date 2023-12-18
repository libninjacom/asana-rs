
use serde::{Serialize, Deserialize};
use super::MembershipResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateMembershipResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<MembershipResponse>,
}
impl std::fmt::Display for CreateMembershipResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}