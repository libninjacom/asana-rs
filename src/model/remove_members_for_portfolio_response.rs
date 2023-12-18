
use serde::{Serialize, Deserialize};
use super::PortfolioResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct RemoveMembersForPortfolioResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<PortfolioResponse>,
}
impl std::fmt::Display for RemoveMembersForPortfolioResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}