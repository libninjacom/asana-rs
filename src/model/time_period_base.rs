use serde::{Serialize, Deserialize};
use super::TimePeriodCompact;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimePeriodBase {
    #[serde(flatten)]
    pub time_period_compact: TimePeriodCompact,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<serde_json::Value>,
}
impl std::fmt::Display for TimePeriodBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for TimePeriodBase {
    type Target = TimePeriodCompact;
    fn deref(&self) -> &Self::Target {
        &self.time_period_compact
    }
}
impl std::ops::DerefMut for TimePeriodBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.time_period_compact
    }
}