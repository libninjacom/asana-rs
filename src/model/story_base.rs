
use serde::{Serialize, Deserialize};
use super::AsanaResource;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StoryBase {
    ///A generic Asana Resource, containing a globally unique identifier.
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    ///The time at which this resource was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    ///[Opt In](/docs/inputoutput-options). HTML formatted text for a comment. This will not include the name of the creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_text: Option<String>,
    ///*Conditional*. Whether the story should be pinned on the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_pinned: Option<bool>,
    ///The subtype of this resource. Different subtypes retain many of the same fields and behavior, but may render differently in Asana or represent resources with different semantic meaning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_subtype: Option<String>,
    ///The name of the sticker in this story. `null` if there is no sticker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_name: Option<String>,
    ///The plain text of the comment to add. Cannot be used with html_text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
impl std::fmt::Display for StoryBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for StoryBase {
    type Target = AsanaResource;
    fn deref(&self) -> &Self::Target {
        &self.asana_resource
    }
}
impl std::ops::DerefMut for StoryBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asana_resource
    }
}