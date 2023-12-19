
use serde::{Serialize, Deserialize};
///A generic Asana Resource, containing a globally unique identifier.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AsanaResource {
    ///Globally unique identifier of the resource, as a string.
    pub gid: String,
    ///The base type of this resource.
    pub resource_type: String,
}
impl std::fmt::Display for AsanaResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}