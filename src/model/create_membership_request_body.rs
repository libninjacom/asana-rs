
use serde::{Serialize, Deserialize};
use super::MembershipRequest;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMembershipRequestBody {
    #[serde(flatten)]
    pub membership_request: MembershipRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
impl std::fmt::Display for CreateMembershipRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for CreateMembershipRequestBody {
    type Target = MembershipRequest;
    fn deref(&self) -> &Self::Target {
        &self.membership_request
    }
}
impl std::ops::DerefMut for CreateMembershipRequestBody {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.membership_request
    }
}