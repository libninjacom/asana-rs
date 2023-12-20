use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskTemplateInstantiateTaskRequest {
    ///The name of the new task. If not provided, the name of the task template will be used.
    pub name: String,
}
impl std::fmt::Display for TaskTemplateInstantiateTaskRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}