use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectDuplicateRequest {
    /**A comma-separated list of elements that will be duplicated to the new project. Tasks are always included.
##### Fields
- forms
- members
- notes
- task_assignee
- task_attachments
- task_dates
- task_dependencies
- task_followers
- task_notes
- task_projects
- task_subtasks
- task_tags*/
    pub include: String,
    ///The name of the new project.
    pub name: String,
    ///A dictionary of options to auto-shift dates. `task_dates` must be included to use this option. Requires either `start_on` or `due_on`, but not both.
    pub schedule_dates: serde_json::Value,
    ///Sets the team of the new project. If team is not defined, the new project will be in the same team as the the original project.
    pub team: String,
}
impl std::fmt::Display for ProjectDuplicateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}