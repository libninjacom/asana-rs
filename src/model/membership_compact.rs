
use serde::{Serialize, Deserialize};
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipCompact(pub serde_json::Value);