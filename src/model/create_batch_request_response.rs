
use serde::{Serialize, Deserialize};
use super::BatchResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct CreateBatchRequestResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<BatchResponse>>,
}
impl std::fmt::Display for CreateBatchRequestResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}