
use serde::{Serialize, Deserialize};
///The primary object that was affected by this event.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuditLogEventResource {
    ///The email of the resource, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    ///Globally unique identifier of the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<String>,
    ///The name of the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The subtype of resource. Most resources will not have a subtype.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_subtype: Option<String>,
    ///The type of resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}
impl std::fmt::Display for AuditLogEventResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}