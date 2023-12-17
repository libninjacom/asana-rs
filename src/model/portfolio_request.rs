
use serde::{Serialize, Deserialize};
use super::PortfolioBase;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioRequest {
    #[serde(flatten)]
    pub portfolio_base: PortfolioBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
}
impl std::fmt::Display for PortfolioRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PortfolioRequest {
    type Target = PortfolioBase;
    fn deref(&self) -> &Self::Target {
        &self.portfolio_base
    }
}
impl std::ops::DerefMut for PortfolioRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.portfolio_base
    }
}