
use serde::{Serialize, Deserialize};
use super::PortfolioCompact;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioBase {
    #[serde(flatten)]
    pub portfolio_compact: PortfolioCompact,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}
impl std::fmt::Display for PortfolioBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PortfolioBase {
    type Target = PortfolioCompact;
    fn deref(&self) -> &Self::Target {
        &self.portfolio_compact
    }
}
impl std::ops::DerefMut for PortfolioBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.portfolio_compact
    }
}