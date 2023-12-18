
use serde::{Serialize, Deserialize};
use super::{ProjectCompact, SectionBase};
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct SectionResponse {
    #[serde(flatten)]
    pub section_base: SectionBase,
    ///The time at which this resource was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectCompact>,
    ///*Deprecated - please use project instead*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<ProjectCompact>>,
}
impl std::fmt::Display for SectionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for SectionResponse {
    type Target = SectionBase;
    fn deref(&self) -> &Self::Target {
        &self.section_base
    }
}
impl std::ops::DerefMut for SectionResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.section_base
    }
}