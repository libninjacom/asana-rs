
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RequestedRoleRequest {
    ///Globally unique identifier of the template role in the project template.
    pub gid: String,
    ///The user id that should be assigned to the template role.
    pub value: String,
}
impl std::fmt::Display for RequestedRoleRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}