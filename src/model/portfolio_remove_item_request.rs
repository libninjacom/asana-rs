use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortfolioRemoveItemRequest {
    ///The item to remove from the portfolio.
    pub item: String,
}
impl std::fmt::Display for PortfolioRemoveItemRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}