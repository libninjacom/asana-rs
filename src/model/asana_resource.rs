
use serde::{Serialize, Deserialize};
use fake::Dummy;
///A generic Asana Resource, containing a globally unique identifier.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct AsanaResource {
    ///Globally unique identifier of the resource, as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<String>,
    ///The base type of this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}
impl std::fmt::Display for AsanaResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}