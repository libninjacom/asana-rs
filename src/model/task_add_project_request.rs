use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskAddProjectRequest {
    ///A task in the project to insert the task after, or `null` to insert at the beginning of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_after: Option<String>,
    ///A task in the project to insert the task before, or `null` to insert at the end of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_before: Option<String>,
    ///The project to add the task to.
    pub project: String,
    ///A section in the project to insert the task into. The task will be inserted at the bottom of the section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
}
impl std::fmt::Display for TaskAddProjectRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}