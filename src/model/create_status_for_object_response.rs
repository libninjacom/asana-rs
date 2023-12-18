
use serde::{Serialize, Deserialize};
use super::StatusUpdateResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateStatusForObjectResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<StatusUpdateResponse>,
}
impl std::fmt::Display for CreateStatusForObjectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}