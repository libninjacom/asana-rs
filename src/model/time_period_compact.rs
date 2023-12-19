
use serde::{Serialize, Deserialize};
use super::AsanaResource;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimePeriodCompact {
    ///A generic Asana Resource, containing a globally unique identifier.
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    ///A string representing the cadence code and the fiscal year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    ///The localized end date of the time period in `YYYY-MM-DD` format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_on: Option<String>,
    ///The cadence and index of the time period. The value is one of: `FY`, `H1`, `H2`, `Q1`, `Q2`, `Q3`, or `Q4`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    ///The localized start date of the time period in `YYYY-MM-DD` format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_on: Option<String>,
}
impl std::fmt::Display for TimePeriodCompact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for TimePeriodCompact {
    type Target = AsanaResource;
    fn deref(&self) -> &Self::Target {
        &self.asana_resource
    }
}
impl std::ops::DerefMut for TimePeriodCompact {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asana_resource
    }
}