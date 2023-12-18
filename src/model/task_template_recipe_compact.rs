
use serde::{Serialize, Deserialize};
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct TaskTemplateRecipeCompact {
    ///Name of the task that will be created from this template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The subtype of the task that will be created from this template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_resource_subtype: Option<String>,
}
impl std::fmt::Display for TaskTemplateRecipeCompact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}