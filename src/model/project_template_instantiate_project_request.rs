
use serde::{Serialize, Deserialize};
use super::{DateVariableRequest, RequestedRoleRequest};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectTemplateInstantiateProjectRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_strict: Option<bool>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_dates: Option<Vec<DateVariableRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_roles: Option<Vec<RequestedRoleRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
}
impl std::fmt::Display for ProjectTemplateInstantiateProjectRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}