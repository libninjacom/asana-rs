use serde::{Serialize, Deserialize};
use super::AttachmentBase;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttachmentResponse {
    #[serde(flatten)]
    pub attachment_base: AttachmentBase,
    ///Whether the attachment is connected to the app making the request for the purposes of showing an app components widget. Only present when the `resource_subtype` is `external` or `gdrive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_to_app: Option<bool>,
    ///The time at which this resource was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**The URL containing the content of the attachment.
*Note:* May be null if the attachment is hosted by [Box](https://www.box.com/) and will be null if the attachment is a Video Message hosted by [Vimeo](https://vimeo.com/). If present, this URL may only be valid for two minutes from the time of retrieval. You should avoid persisting this URL somewhere and just refresh it on demand to ensure you do not keep stale URLs.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    ///The service hosting the attachment. Valid values are `asana`, `dropbox`, `gdrive`, `box`, and `vimeo`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<serde_json::Value>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent_url: Option<String>,
    ///The size of the attachment in bytes. Only present when the `resource_subtype` is `asana`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    ///The URL where the attachment can be viewed, which may be friendlier to users in a browser than just directing them to a raw file. May be null if no view URL exists for the service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_url: Option<String>,
}
impl std::fmt::Display for AttachmentResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for AttachmentResponse {
    type Target = AttachmentBase;
    fn deref(&self) -> &Self::Target {
        &self.attachment_base
    }
}
impl std::ops::DerefMut for AttachmentResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attachment_base
    }
}
