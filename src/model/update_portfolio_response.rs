
use serde::{Serialize, Deserialize};
use super::PortfolioResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdatePortfolioResponse {
    pub data: PortfolioResponse,
}
impl std::fmt::Display for UpdatePortfolioResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}