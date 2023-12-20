use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SectionTaskInsertRequest {
    ///An existing task within this section after which the added task should be inserted. Cannot be provided together with insert_before.
    pub insert_after: String,
    ///An existing task within this section before which the added task should be inserted. Cannot be provided together with insert_after.
    pub insert_before: String,
    ///The task to add to this section.
    pub task: String,
}
impl std::fmt::Display for SectionTaskInsertRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}