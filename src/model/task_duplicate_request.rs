
use serde::{Serialize, Deserialize};
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
    ///The name of the new task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for TaskDuplicateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}