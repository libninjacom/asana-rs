use serde::{Serialize, Deserialize};
use super::BatchRequestAction;
///A request object for use in a batch request.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchRequest {
    pub actions: Vec<BatchRequestAction>,
}
impl std::fmt::Display for BatchRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
