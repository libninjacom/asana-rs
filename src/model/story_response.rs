
use serde::{Serialize, Deserialize};
use super::{
    CustomFieldCompact, EnumOption, Like, Preview, ProjectCompact, SectionCompact,
    StoryBase, StoryCompact, StoryResponseDates, TagCompact, TaskCompact, UserCompact,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryResponse {
    #[serde(flatten)]
    pub story_base: StoryBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<UserCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field: Option<CustomFieldCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency: Option<TaskCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_of: Option<TaskCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicated_from: Option<TaskCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follower: Option<UserCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hearted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hearts: Option<Vec<Like>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_editable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_edited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes: Option<Vec<Like>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_approval_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_date_value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_dates: Option<StoryResponseDates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_enum_value: Option<EnumOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_multi_enum_values: Option<Vec<EnumOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_number_value: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_people_value: Option<Vec<UserCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_resource_subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_section: Option<SectionCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_text_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_hearts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_likes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_approval_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_date_value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_dates: Option<StoryResponseDates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_enum_value: Option<EnumOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_multi_enum_values: Option<Vec<EnumOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_number_value: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_people_value: Option<Vec<UserCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_resource_subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_section: Option<SectionCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_text_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previews: Option<Vec<Preview>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<StoryCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<TagCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<TaskCompact>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for StoryResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for StoryResponse {
    type Target = StoryBase;
    fn deref(&self) -> &Self::Target {
        &self.story_base
    }
}
impl std::ops::DerefMut for StoryResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.story_base
    }
}