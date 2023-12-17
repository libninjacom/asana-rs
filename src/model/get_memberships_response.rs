
use serde::{Serialize, Deserialize};
use super::{MembershipCompact, NextPage};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetMembershipsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<MembershipCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<NextPage>,
}
impl std::fmt::Display for GetMembershipsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}