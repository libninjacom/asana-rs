use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortfolioAddItemRequest {
    ///An id of an item in this portfolio. The new item will be added after the one specified here. `insert_before` and `insert_after` parameters cannot both be specified.
    pub insert_after: String,
    ///An id of an item in this portfolio. The new item will be added before the one specified here. `insert_before` and `insert_after` parameters cannot both be specified.
    pub insert_before: String,
    ///The item to add to the portfolio.
    pub item: String,
}
impl std::fmt::Display for PortfolioAddItemRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}