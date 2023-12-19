
use serde::{Serialize, Deserialize};
///A response object returned from a batch request.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchResponse {
    ///The JSON body that the invoked endpoint returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    ///A map of HTTP headers specific to this result. This is primarily used for returning a `Location` header to accompany a `201 Created` result.  The parent HTTP response will contain all common headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    ///The HTTP status code that the invoked endpoint returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}
impl std::fmt::Display for BatchResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}