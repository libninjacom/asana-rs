use serde::{Serialize, Deserialize};
///A response object returned from a batch request.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchResponse {
    ///The JSON body that the invoked endpoint returned.
    pub body: serde_json::Value,
    ///A map of HTTP headers specific to this result. This is primarily used for returning a `Location` header to accompany a `201 Created` result.  The parent HTTP response will contain all common headers.
    pub headers: serde_json::Value,
    ///The HTTP status code that the invoked endpoint returned.
    pub status_code: i64,
}
impl std::fmt::Display for BatchResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}