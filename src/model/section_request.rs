
use serde::{Serialize, Deserialize};
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct SectionRequest {
    ///An existing section within this project after which the added section should be inserted. Cannot be provided together with insert_before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_after: Option<String>,
    ///An existing section within this project before which the added section should be inserted. Cannot be provided together with insert_after.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_before: Option<String>,
    ///The text to be displayed as the section name. This cannot be an empty string.
    pub name: String,
}
impl std::fmt::Display for SectionRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}