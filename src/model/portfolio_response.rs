
use serde::{Serialize, Deserialize};
use super::{
    CustomFieldCompact, CustomFieldSettingResponse, PortfolioBase,
    ProjectTemplateCompact, UserCompact,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioResponse {
    #[serde(flatten)]
    pub portfolio_base: PortfolioBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_status_update: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_settings: Option<Vec<CustomFieldSettingResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_on: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<UserCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<UserCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permalink_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_templates: Option<Vec<ProjectTemplateCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
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