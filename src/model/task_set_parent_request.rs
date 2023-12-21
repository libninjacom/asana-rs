use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskSetParentRequest {
    ///A subtask of the parent to insert the task after, or `null` to insert at the beginning of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_after: Option<String>,
    ///A subtask of the parent to insert the task before, or `null` to insert at the end of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_before: Option<String>,
    ///The new parent of the task, or `null` for no parent.
    pub parent: String,
}
impl std::fmt::Display for TaskSetParentRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}