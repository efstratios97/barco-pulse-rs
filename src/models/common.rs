#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct JsonRpcPayload {
    pub jsonrpc: String,
    pub method: String,
    pub id: String,
    pub params: std::collections::HashMap<String, String>,
}
