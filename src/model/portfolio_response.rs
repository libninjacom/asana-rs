
use serde::{Serialize, Deserialize};
use super::{
    CustomFieldCompact, CustomFieldSettingResponse, PortfolioBase,
    ProjectTemplateCompact, UserCompact,
};
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct PortfolioResponse {
    #[serde(flatten)]
    pub portfolio_base: PortfolioBase,
    ///The time at which this resource was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_status_update: Option<serde_json::Value>,
    ///Array of custom field settings applied to the portfolio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_settings: Option<Vec<CustomFieldSettingResponse>>,
    ///Array of Custom Fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldCompact>>,
    ///The localized day on which this portfolio is due. This takes a date with format YYYY-MM-DD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_on: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<UserCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<UserCompact>,
    ///A url that points directly to the object within Asana.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permalink_url: Option<String>,
    ///Array of project templates that are in the portfolio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_templates: Option<Vec<ProjectTemplateCompact>>,
    ///True if the portfolio is public to its workspace members.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    ///The day on which work for this portfolio begins, or null if the portfolio has no start date. This takes a date with `YYYY-MM-DD` format. *Note: `due_on` must be present in the request when setting or unsetting the `start_on` parameter. Additionally, `start_on` and `due_on` cannot be the same date.*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_on: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<serde_json::Value>,
}
impl std::fmt::Display for PortfolioResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PortfolioResponse {
    type Target = PortfolioBase;
    fn deref(&self) -> &Self::Target {
        &self.portfolio_base
    }
}
impl std::ops::DerefMut for PortfolioResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.portfolio_base
    }
}