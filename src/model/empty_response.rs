
use serde::{Serialize, Deserialize};
use fake::Dummy;
///An empty object. Some endpoints do not return an object on success. The success is conveyed through a 2-- status code and returning an empty object.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct EmptyResponse {}
impl std::fmt::Display for EmptyResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}