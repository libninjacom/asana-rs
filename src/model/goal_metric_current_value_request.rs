
use serde::{Serialize, Deserialize};
use super::AsanaResource;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GoalMetricCurrentValueRequest {
    ///A generic Asana Resource, containing a globally unique identifier.
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    ///*Conditional*. This number is the current value of a goal metric of type number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_number_value: Option<f64>,
}
impl std::fmt::Display for GoalMetricCurrentValueRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for GoalMetricCurrentValueRequest {
    type Target = AsanaResource;
    fn deref(&self) -> &Self::Target {
        &self.asana_resource
    }
}
impl std::ops::DerefMut for GoalMetricCurrentValueRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asana_resource
    }
}