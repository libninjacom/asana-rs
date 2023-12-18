
use serde::{Serialize, Deserialize};
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct RequestedRoleRequest {
    ///Globally unique identifier of the template role in the project template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<String>,
    ///The user id that should be assigned to the template role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl std::fmt::Display for RequestedRoleRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}