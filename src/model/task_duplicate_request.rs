use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskDuplicateRequest {
    /**A comma-separated list of fields that will be duplicated to the new task.
##### Fields
- assignee
- attachments
- dates
- dependencies
- followers
- notes
- parent
- projects
- subtasks
- tags*/
    pub include: String,
    ///The name of the new task.
    pub name: String,
}
impl std::fmt::Display for TaskDuplicateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
