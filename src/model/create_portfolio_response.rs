use serde::{Serialize, Deserialize};
use super::PortfolioResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreatePortfolioResponse {
    pub data: PortfolioResponse,
}
impl std::fmt::Display for CreatePortfolioResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}