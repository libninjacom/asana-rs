// libninja: static
use serde::{Serialize, Deserialize};
///A generic Asana Resource, containing a globally unique identifier.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AsanaResource {
    ///Globally unique identifier of the resource, as a string.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub gid: String,
    ///The base type of this resource.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub resource_type: String,
}
impl std::fmt::Display for AsanaResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}