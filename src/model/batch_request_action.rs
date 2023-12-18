
use serde::{Serialize, Deserialize};
use fake::Dummy;
///An action object for use in a batch request.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct BatchRequestAction {
    ///For `GET` requests, this should be a map of query parameters you would have normally passed in the URL. Options and pagination are not accepted here; put them in `options` instead. For `POST`, `PATCH`, and `PUT` methods, this should be the content you would have normally put in the data field of the body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    ///The HTTP method you wish to emulate for the action.
    pub method: String,
    ///Pagination (`limit` and `offset`) and output options (`fields` or `expand`) for the action. “Pretty” JSON output is not an available option on individual actions; if you want pretty output, specify that option on the parent request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    ///The path of the desired endpoint relative to the API’s base URL. Query parameters are not accepted here; put them in `data` instead.
    pub relative_path: String,
}
impl std::fmt::Display for BatchRequestAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}