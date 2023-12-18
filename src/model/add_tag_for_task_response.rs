
use serde::{Serialize, Deserialize};
use super::EmptyResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct AddTagForTaskResponse {
    ///An empty object. Some endpoints do not return an object on success. The success is conveyed through a 2-- status code and returning an empty object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<EmptyResponse>,
}
impl std::fmt::Display for AddTagForTaskResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}