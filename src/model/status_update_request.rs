use serde::{Serialize, Deserialize};
use super::StatusUpdateBase;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatusUpdateRequest {
    #[serde(flatten)]
    pub status_update_base: StatusUpdateBase,
    pub parent: String,
}
impl std::fmt::Display for StatusUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for StatusUpdateRequest {
    type Target = StatusUpdateBase;
    fn deref(&self) -> &Self::Target {
        &self.status_update_base
    }
}
impl std::ops::DerefMut for StatusUpdateRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.status_update_base
    }
}