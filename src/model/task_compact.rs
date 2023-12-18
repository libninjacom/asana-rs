
use serde::{Serialize, Deserialize};
use super::AsanaResource;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct TaskCompact {
    ///A generic Asana Resource, containing a globally unique identifier.
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    ///[Opt In](/docs/inputoutput-options). A *user* object represents an account in Asana that can be given access to various workspaces, projects, and tasks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<serde_json::Value>,
    ///The name of the task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /**The subtype of this resource. Different subtypes retain many of the same fields and behavior, but may render differently in Asana or represent resources with different semantic meaning.
The resource_subtype `milestone` represent a single moment in time. This means tasks with this subtype cannot have a start_date.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_subtype: Option<String>,
}
impl std::fmt::Display for TaskCompact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for TaskCompact {
    type Target = AsanaResource;
    fn deref(&self) -> &Self::Target {
        &self.asana_resource
    }
}
impl std::ops::DerefMut for TaskCompact {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asana_resource
    }
}