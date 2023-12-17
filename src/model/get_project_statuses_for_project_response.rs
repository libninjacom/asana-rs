
use serde::{Serialize, Deserialize};
use super::{NextPage, ProjectStatusCompact};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetProjectStatusesForProjectResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ProjectStatusCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<NextPage>,
}
impl std::fmt::Display for GetProjectStatusesForProjectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}