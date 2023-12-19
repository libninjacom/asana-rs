
use serde::{Serialize, Deserialize};
use super::StatusUpdateResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateStatusForObjectResponse {
    pub data: StatusUpdateResponse,
}
impl std::fmt::Display for CreateStatusForObjectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}