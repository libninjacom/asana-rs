use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SectionRequest {
    ///An existing section within this project after which the added section should be inserted. Cannot be provided together with insert_before.
    pub insert_after: String,
    ///An existing section within this project before which the added section should be inserted. Cannot be provided together with insert_after.
    pub insert_before: String,
    ///The text to be displayed as the section name. This cannot be an empty string.
    pub name: String,
}
impl std::fmt::Display for SectionRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}