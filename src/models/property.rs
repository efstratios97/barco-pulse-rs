#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Property {
    pub property: String,
    pub description: String,
    pub access: String,
    pub get: JsonRpcPayload,
    pub set: Option<JsonRpcPayload>,
}
