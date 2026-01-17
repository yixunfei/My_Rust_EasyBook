use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub op: String,
    pub key: String,
    pub value: Option<serde_json::Value>,
    pub ttl: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub ok: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}
