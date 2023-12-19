use serde::{Serialize, Deserialize};
use super::AttachmentResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAttachmentForObjectResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AttachmentResponse>,
}
impl std::fmt::Display for CreateAttachmentForObjectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
