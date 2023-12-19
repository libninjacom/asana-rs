
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DateVariableCompact {
    ///The description of what the date variable is used for when instantiating a project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Globally unique identifier of the date field in the project template. A value of `1` refers to the project start date, while `2` refers to the project due date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<String>,
    ///The name of the date variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for DateVariableCompact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}