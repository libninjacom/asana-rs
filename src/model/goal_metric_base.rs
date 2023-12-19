
use serde::{Serialize, Deserialize};
use super::AsanaResource;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GoalMetricBase {
    ///A generic Asana Resource, containing a globally unique identifier.
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    ///ISO 4217 currency code to format this custom field. This will be null if the `unit` is not `currency`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    ///This string is the current value of a goal metric of type string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_display_value: Option<String>,
    ///This number is the current value of a goal metric of type number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_number_value: Option<f64>,
    ///This number is the start value of a goal metric of type number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_number_value: Option<f64>,
    #[doc = "*Conditional*. Only relevant for goal metrics of type ‘Number’. This field dictates the number of places after the decimal to round to, i.e. 0 is integer values, 1 rounds to the nearest tenth, and so on. Must be between 0 and 6, inclusive.\nFor percentage format, this may be unintuitive, as a value of 0.25 has a precision of 0, while a value of 0.251 has a precision of 1. This is due to 0.25 being displayed as 25%."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<i64>,
    ///This field defines how the progress value of a goal metric is being calculated. A goal's progress can be provided manually by the user, calculated automatically from contributing subgoals, projects, or tasks, or managed by an integration with an external data source, such as Salesforce.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_source: Option<String>,
    ///The subtype of this resource. Different subtypes retain many of the same fields and behavior, but may render differently in Asana or represent resources with different semantic meaning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_subtype: Option<String>,
    ///This number is the end value of a goal metric of type number. This number cannot equal `initial_number_value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_number_value: Option<f64>,
    ///A supported unit of measure for the goal metric, or none.
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