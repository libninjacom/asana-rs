
use serde::{Serialize, Deserialize};
use super::TimePeriodResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTimePeriodResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TimePeriodResponse>,
}
impl std::fmt::Display for GetTimePeriodResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}