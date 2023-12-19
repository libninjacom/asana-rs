use serde::{Serialize, Deserialize};
use super::EmptyResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddItemForPortfolioResponse {
    ///An empty object. Some endpoints do not return an object on success. The success is conveyed through a 2-- status code and returning an empty object.
    pub data: EmptyResponse,
}
impl std::fmt::Display for AddItemForPortfolioResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
