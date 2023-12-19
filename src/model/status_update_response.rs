use serde::{Serialize, Deserialize};
use super::{Like, StatusUpdateBase, UserCompact};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatusUpdateResponse {
    #[serde(flatten)]
    pub status_update_base: StatusUpdateBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<UserCompact>,
    ///The time at which this resource was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserCompact>,
    ///*Deprecated - please use liked instead* True if the status is hearted by the authorized user, false if not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hearted: Option<bool>,
    ///*Deprecated - please use likes instead* Array of likes for users who have hearted this status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hearts: Option<Vec<Like>>,
    ///True if the status is liked by the authorized user, false if not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liked: Option<bool>,
    ///Array of likes for users who have liked this status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes: Option<Vec<Like>>,
    /**The time at which this project status was last modified.
*Note: This does not currently reflect any changes in associations such as comments that may have been added or removed from the status.**/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    ///*Deprecated - please use likes instead* The number of users who have hearted this status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_hearts: Option<i64>,
    ///The number of users who have liked this status.
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
