use serde::{Serialize, Deserialize};
use super::{CustomFieldSettingResponse, ProjectCompact, UserCompact};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectBase {
    #[serde(flatten)]
    pub project_compact: ProjectCompact,
    ///True if the project is archived, false if not. Archived projects do not show in the UI by default and may be treated differently for queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    ///Color of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<serde_json::Value>,
    ///The time at which this resource was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_status: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_status_update: Option<serde_json::Value>,
    ///Array of Custom Field Settings (in compact form).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_settings: Option<Vec<CustomFieldSettingResponse>>,
    ///The default access for users or teams who join or are added as members to the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_access_level: Option<String>,
    ///The default view (list, board, calendar, or timeline) of a project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_view: Option<String>,
    ///*Deprecated: new integrations should prefer the `due_on` field.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<chrono::NaiveDate>,
    ///The day on which this project is due. This takes a date with format YYYY-MM-DD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_on: Option<chrono::NaiveDate>,
    ///[Opt In](/docs/inputoutput-options). The notes of the project with formatting as HTML.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_notes: Option<String>,
    ///Array of users who are members of this project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<UserCompact>>,
    ///The minimum access level needed for project members to modify this project's workflow and appearance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_access_level_for_customization: Option<String>,
    ///The minimum access level needed for project members to share the project and manage project memberships.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_access_level_for_sharing: Option<String>,
    /**The time at which this project was last modified.
*Note: This does not currently reflect any changes in associations such as tasks or comments that may have been added or removed from the project.**/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    ///Free-form textual information associated with the project (ie., its description).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    ///True if the project is public to its team.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    ///The day on which work for this project begins, or null if the project has no start date. This takes a date with `YYYY-MM-DD` format. *Note: `due_on` or `due_at` must be present in the request when setting or unsetting the `start_on` parameter. Additionally, `start_on` and `due_on` cannot be the same date.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_on: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for ProjectBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ProjectBase {
    type Target = ProjectCompact;
    fn deref(&self) -> &Self::Target {
        &self.project_compact
    }
}
impl std::ops::DerefMut for ProjectBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.project_compact
    }
}
