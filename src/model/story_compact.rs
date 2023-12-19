use serde::{Serialize, Deserialize};
use super::{AsanaResource, UserCompact};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StoryCompact {
    ///A generic Asana Resource, containing a globally unique identifier.
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    ///The time at which this resource was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserCompact>,
    ///The subtype of this resource. Different subtypes retain many of the same fields and behavior, but may render differently in Asana or represent resources with different semantic meaning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_subtype: Option<String>,
    #[doc = "*Create-only*. Human-readable text for the story or comment.\nThis will not include the name of the creator.\n*Note: This is not guaranteed to be stable for a given type of story. For example, text for a reassignment may not always say “assigned to …” as the text for a story can both be edited and change based on the language settings of the user making the request.*\nUse the `resource_subtype` property to discover the action that created the story."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
impl std::fmt::Display for StoryCompact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for StoryCompact {
    type Target = AsanaResource;
    fn deref(&self) -> &Self::Target {
        &self.asana_resource
    }
}
impl std::ops::DerefMut for StoryCompact {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asana_resource
    }
}
