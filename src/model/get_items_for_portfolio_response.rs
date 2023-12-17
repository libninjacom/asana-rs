
use serde::{Serialize, Deserialize};
use super::{NextPage, ProjectCompact};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetItemsForPortfolioResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ProjectCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<NextPage>,
}
impl std::fmt::Display for GetItemsForPortfolioResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}