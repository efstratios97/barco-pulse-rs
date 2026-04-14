#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Method {
    pub method: String,
    pub description: String,
    pub request: JsonRpcPayload,
}
