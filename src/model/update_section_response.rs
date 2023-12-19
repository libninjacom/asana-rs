
use serde::{Serialize, Deserialize};
use super::SectionResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSectionResponse {
    pub data: SectionResponse,
}
impl std::fmt::Display for UpdateSectionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}