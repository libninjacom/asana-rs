
use serde::{Serialize, Deserialize};
use super::{DateVariableRequest, RequestedRoleRequest};
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct ProjectTemplateInstantiateProjectRequest {
    ///*Optional*. If set to `true`, the endpoint returns an "Unprocessable Entity" error if you fail to provide a calendar date value for any date variable. If set to `false`, a default date is used for each unfulfilled date variable (e.g., the current date is used as the Start Date of a project).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_strict: Option<bool>,
    ///The name of the new project.
    pub name: String,
    ///Sets the project to public to its team.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    ///Array of mappings of date variables to calendar dates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_dates: Option<Vec<DateVariableRequest>>,
    ///Array of mappings of template roles to user ids
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_roles: Option<Vec<RequestedRoleRequest>>,
    ///*Optional*. Sets the team of the new project. If the project template exists in an _organization_, you may specify a value for `team`. If no value is provided then it defaults to the same team as the project template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
}
impl std::fmt::Display for ProjectTemplateInstantiateProjectRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}