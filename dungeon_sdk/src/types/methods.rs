use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "method", content = "data")]
pub enum Method {
    /// Choosed login
    Connect((String, String)),
    /// Get player statistics
    GetStats,
    /// Just Ping
    Ping,
}
