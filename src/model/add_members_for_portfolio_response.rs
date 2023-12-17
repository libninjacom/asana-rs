
use serde::{Serialize, Deserialize};
use super::PortfolioResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddMembersForPortfolioResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<PortfolioResponse>,
}
impl std::fmt::Display for AddMembersForPortfolioResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}