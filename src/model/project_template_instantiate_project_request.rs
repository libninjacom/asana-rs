use serde::{Serialize, Deserialize};
use super::{DateVariableRequest, RequestedRoleRequest};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectTemplateInstantiateProjectRequest {
    ///*Optional*. If set to `true`, the endpoint returns an "Unprocessable Entity" error if you fail to provide a calendar date value for any date variable. If set to `false`, a default date is used for each unfulfilled date variable (e.g., the current date is used as the Start Date of a project).
    pub is_strict: bool,
    ///The name of the new project.
    pub name: String,
    ///Sets the project to public to its team.
    pub public: bool,
    ///Array of mappings of date variables to calendar dates.
    pub requested_dates: Vec<DateVariableRequest>,
    ///Array of mappings of template roles to user ids
    pub requested_roles: Vec<RequestedRoleRequest>,
    ///*Optional*. Sets the team of the new project. If the project template exists in an _organization_, you may specify a value for `team`. If no value is provided then it defaults to the same team as the project template.
    pub team: String,
}
impl std::fmt::Display for ProjectTemplateInstantiateProjectRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}