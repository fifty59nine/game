use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub user: String,
    pub body: crate::types::Method,
}

impl Request {
    pub fn new(body: crate::types::Method, user: Option<String>) -> Self {
        Self {
            user: user.unwrap_or_default(),
            body,
        }
    }
}
