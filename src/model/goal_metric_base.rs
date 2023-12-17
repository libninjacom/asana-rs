
use serde::{Serialize, Deserialize};
use super::AsanaResource;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalMetricBase {
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_display_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_number_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_number_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_number_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
impl std::fmt::Display for GoalMetricBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for GoalMetricBase {
    type Target = AsanaResource;
    fn deref(&self) -> &Self::Target {
        &self.asana_resource
    }
}
impl std::ops::DerefMut for GoalMetricBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asana_resource
    }
}