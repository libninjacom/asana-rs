
use serde::{Serialize, Deserialize};
use super::{Like, StatusUpdateBase, UserCompact};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusUpdateResponse {
    #[serde(flatten)]
    pub status_update_base: StatusUpdateBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<UserCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hearted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hearts: Option<Vec<Like>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes: Option<Vec<Like>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_hearts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_likes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<serde_json::Value>,
}
impl std::fmt::Display for StatusUpdateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for StatusUpdateResponse {
    type Target = StatusUpdateBase;
    fn deref(&self) -> &Self::Target {
        &self.status_update_base
    }
}
impl std::ops::DerefMut for StatusUpdateResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.status_update_base
    }
}