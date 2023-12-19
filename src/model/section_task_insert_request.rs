
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SectionTaskInsertRequest {
    ///An existing task within this section after which the added task should be inserted. Cannot be provided together with insert_before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_after: Option<String>,
    ///An existing task within this section before which the added task should be inserted. Cannot be provided together with insert_after.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_before: Option<String>,
    ///The task to add to this section.
    pub task: String,
}
impl std::fmt::Display for SectionTaskInsertRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}