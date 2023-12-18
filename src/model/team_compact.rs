
use serde::{Serialize, Deserialize};
use super::AsanaResource;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct TeamCompact {
    ///A generic Asana Resource, containing a globally unique identifier.
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    ///The name of the team.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for TeamCompact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for TeamCompact {
    type Target = AsanaResource;
    fn deref(&self) -> &Self::Target {
        &self.asana_resource
    }
}
impl std::ops::DerefMut for TeamCompact {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asana_resource
    }
}