
use serde::{Serialize, Deserialize};
use super::{NextPage, PortfolioMembershipCompact};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetPortfolioMembershipsForPortfolioResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<PortfolioMembershipCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<NextPage>,
}
impl std::fmt::Display for GetPortfolioMembershipsForPortfolioResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}