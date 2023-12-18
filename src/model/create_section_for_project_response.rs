
use serde::{Serialize, Deserialize};
use super::SectionResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct CreateSectionForProjectResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<SectionResponse>,
}
impl std::fmt::Display for CreateSectionForProjectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}