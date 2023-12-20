use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DateVariableRequest {
    ///Globally unique identifier of the date field in the project template. A value of `1` refers to the project start date, while `2` refers to the project due date.
    pub gid: String,
    ///The date with which the date variable should be replaced when instantiating a project. This takes a date with `YYYY-MM-DD` format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for DateVariableRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}