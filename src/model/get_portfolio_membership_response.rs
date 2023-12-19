
use serde::{Serialize, Deserialize};
use super::PortfolioMembershipResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetPortfolioMembershipResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<PortfolioMembershipResponse>,
}
impl std::fmt::Display for GetPortfolioMembershipResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}