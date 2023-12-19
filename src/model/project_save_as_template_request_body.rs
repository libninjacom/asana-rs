
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectSaveAsTemplateRequestBody {
    ///The name of the new project template.
    pub name: String,
    ///Sets the project template to public to its team.
    pub public: bool,
    ///Sets the team of the new project template. If the project exists in an organization, specify team and not workspace.
    pub team: String,
    ///Sets the workspace of the new project template. Only specify workspace if the project exists in a workspace.
    pub workspace: String,
}
impl std::fmt::Display for ProjectSaveAsTemplateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}