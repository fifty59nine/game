use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub proto_id: u64,
    pub user: String,
    pub body: crate::types::Method,
}

impl Request {
    pub fn new(body: crate::types::Method, proto_id: u64, user: Option<String>) -> Self {
        Self {
            proto_id,
            user: user.unwrap_or_default(),
            body,
        }
    }
}
