use std::path::Path;
use std::{ fs::{ File, write, metadata }, io::BufReader };
use serde::de::DeserializeOwned;

// Iclude property models to map the json
include!("./src/models/common.rs");
include!("./src/models/property.rs");
include!("./src/models/method.rs");

pub type ApiFilesResult<T> = Result<Vec<T>, Box<dyn std::error::Error>>;

///////////////////////////////////////////////////////////////////////////
/// COMMON ////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////
const HEADER: &str = "
// Copyright (c) 2026 Efstratios Pahis
// SPDX-License-Identifier: MPL-2.0
";

const COMMON_INIT_CODE: &str =
    r#"
#![allow(unused_variables)]
#![allow(non_snake_case)]


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct APICallResponse {
    pub jsonrpc: String,
    #[serde(default)]
    pub result: serde_json::Value,
    #[serde(default)]
    pub error: serde_json::Value,
    pub id: serde_json::Value,
}

type APICallResult = Result<APICallResponse, Box<dyn std::error::Error>>;

pub struct PulseSession {
    stream: tokio::net::TcpStream,
}

impl PulseSession {
    pub async fn connect(address: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            stream: tokio::net::TcpStream::connect(address).await?,
        })
    }

    pub async fn call(
        &mut self,
        payload: serde_json::Value,
    ) -> APICallResult {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let mut payload_string = payload.to_string();
        payload_string.push('\n');

        self.stream.write_all(payload_string.as_bytes()).await?;
        self.stream.flush().await?;

        let mut buf = vec![0_u8; 8192];

        let n = tokio::time::timeout(
            tokio::time::Duration::from_secs(15),
            self.stream.read(&mut buf)
        ).await??;

        let body = String::from_utf8_lossy(&buf[..n]).to_string();

        println!("Pulse response: {:?}", body);

        let res: APICallResponse = serde_json::from_str(&body)?;
        Ok(res)
    }
}
"#;

fn read_json_file<T>(file_path: &str) -> ApiFilesResult<T> where T: DeserializeOwned {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let json_file: Vec<T> = serde_json::from_reader(reader)?;

    Ok(json_file)
}

fn write_to_file(code: String, file_path: &str) {
    let dest_path = Path::new(file_path);
    let should_write =
        !dest_path.exists() ||
        metadata(dest_path)
            .map(|m| m.len() == 0)
            .unwrap_or(false);

    if should_write {
        write(dest_path, code).unwrap();
    }
}

fn match_set_value_type(rpc_data: &Option<JsonRpcPayload>, param_key: &str) -> Option<String> {
    let params = rpc_data.as_ref().unwrap().params.get(param_key);

    if params.is_none() {
        None
    } else {
        match params.unwrap().as_str() {
            "<bool>" => Some(String::from("bool")),
            "<float>" => Some(String::from("f64")),
            "<int>" => Some(String::from("u64")),
            "<object>" => Some(String::from("std::collections::HashMap<String, String>")),
            "[<string>]" => Some(String::from("std::vec::Vec<String>")),
            "[<object>]" =>
                Some(String::from("std::vec::Vec<std::collections::HashMap<String, String>>")),
            _ => Some(String::from("String")),
        }
    }
}

fn create_lib_rs_file() {
    let generated_code = String::from("pub mod method_api;
pub mod property_api;
");
    write_to_file(generated_code, "./src/lib.rs");
}

///////////////////////////////////////////////////////////////////////////
/// Barco Property API ////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////
fn create_property_fns(properties: Vec<Property>) {
    let mut generated_code = String::from(HEADER);
    generated_code.push_str(COMMON_INIT_CODE);

    properties
        .iter()
        .for_each(|property| {
            generated_code.push_str(&get_jsonrpc_generated_properties_code(property))
        });

    write_to_file(generated_code, "./src/property_api.rs");
}

fn get_fn_signature_by_property_type(property: &Property) -> String {
    let property_name = property.property.clone().replace(".", "_").replace("-", "_");

    if property.access == "R" {
        String::from("get_") + &property_name + "(session: &mut PulseSession)"
    } else {
        String::from("set_") +
            &property_name +
            format!(
                "(\n    session: &mut PulseSession,\n    value: {})",
                match_set_value_type(&property.set, "value").unwrap()
            ).as_str()
    }
}

fn get_jsonrpc_generated_properties_code(property: &Property) -> String {
    fn payload_id_helper(payload: &mut JsonRpcPayload, property: &Property) -> JsonRpcPayload {
        payload.id = payload.id.replace("<number|string>", property.property.as_str());
        payload.clone()
    }

    let mut payload = property.get.clone();
    payload_id_helper(&mut payload, &property);
    let fn_signature: String = get_fn_signature_by_property_type(&property);

    if property.access == "RW" {
        payload = property.set.clone().unwrap();
        payload_id_helper(&mut payload, &property);

        format!(
            r#"///{}
pub async fn {} -> APICallResult {{
    let payload = serde_json::json!({{
        "jsonrpc": "{}",
        "method": "{}",
        "id": "{}",
        "params": {{
            "property": "{}",
            "value": value
        }}
    }});

    session.call(payload).await
}}
"#,
            &property.description,
            fn_signature,
            payload.jsonrpc,
            payload.method,
            payload.id,
            payload.params.get("property").unwrap()
        )
    } else {
        format!(
            r#"///{}
pub async fn {} -> APICallResult {{
    let payload = serde_json::json!({});

    session.call(payload).await
}}
"#,
            &property.description,
            get_fn_signature_by_property_type(&property).as_str(),
            serde_json::to_string(&payload).unwrap()
        )
    }
}

///////////////////////////////////////////////////////////////////////////
/// Barco Method API //////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////
fn create_method_fns(methods: Vec<Method>) {
    let mut generated_code = String::from(HEADER);
    generated_code.push_str(COMMON_INIT_CODE);

    generated_code.push_str(
        r#"/// custom pulse method api call
pub async fn custom_method_call(
    session: &mut PulseSession,
    method_name: &str,
    params: std::collections::HashMap<String, String>,
) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method_name,
        "id": method_name,
        "params": params
    });

    session.call(payload).await
}
"#
    );

    methods
        .iter()
        .for_each(|method| generated_code.push_str(&get_jsonrpc_generated_methods_code(method)));

    write_to_file(generated_code, "./src/method_api.rs");
}

fn get_fn_signature_by_method_type(method: &Method) -> (String, Option<String>) {
    let method_name = method.method.clone().replace(".", "_").replace("-", "_").replace(" ", "");

    let mut param_signature: Option<String> = None;
    let mut param_signature_tmp = String::from("");

    let mut body_params: Option<String> = None;
    let mut body_params_tmp = String::from("");

    method.request.params.iter().for_each(|(param_key, _)| {
        let param_key = if param_key == "type" { "r#type" } else { param_key };

        let param_value_type = match_set_value_type(&Some(method.request.clone()), param_key);

        if param_value_type.is_some() {
            param_signature_tmp += &format!("{}: {},", param_key, param_value_type.unwrap());
            body_params_tmp += &format!("\"{}\": {},", param_key, param_key);
        }

        param_signature = Some(param_signature_tmp.clone());
        body_params = Some(body_params_tmp.clone());
    });

    if param_signature.is_some() {
        (
            String::from("set_") +
                &method_name +
                format!(
                    "(\n    session: &mut PulseSession,\n    {})",
                    param_signature.unwrap()
                ).as_str(),
            body_params,
        )
    } else {
        (String::from("set_") + &method_name + "(session: &mut PulseSession)", body_params)
    }
}

fn get_jsonrpc_generated_methods_code(method: &Method) -> String {
    fn payload_id_helper(payload: &mut JsonRpcPayload, method: &Method) -> JsonRpcPayload {
        payload.id = payload.id.replace("<number|string>", method.method.as_str());
        payload.clone()
    }

    let mut payload = method.request.clone();
    payload_id_helper(&mut payload, &method);

    let fn_parts = get_fn_signature_by_method_type(&method);
    let fn_signature = fn_parts.0.as_str();
    let fn_body = fn_parts.1;

    if fn_body.is_some() {
        format!(
            r#"///{}
pub async fn {} -> APICallResult {{
    let payload = serde_json::json!({{
        "jsonrpc": "{}",
        "method": "{}",
        "id": "{}",
        "params": {{{}}}
    }});

    session.call(payload).await
}}
"#,
            &method.description,
            fn_signature,
            payload.jsonrpc,
            payload.method,
            payload.id,
            fn_body.as_ref().unwrap()
        )
    } else {
        format!(
            r#"///{}
pub async fn {} -> APICallResult {{
    let payload = serde_json::json!({});

    session.call(payload).await
}}
"#,
            &method.description,
            fn_signature,
            serde_json::to_string(&payload).unwrap()
        )
    }
}

fn main() {
    let file_path_properties = "./src/barco_api/properties.json";
    let properties: Vec<Property> = read_json_file(file_path_properties).unwrap_or_else(|err| {
        panic!("{:?}", err);
    });
    create_property_fns(properties);

    let file_path_methods = "./src/barco_api/methods.json";
    let methods: Vec<Method> = read_json_file(file_path_methods).unwrap_or_else(|err| {
        panic!("{:?}", err);
    });
    create_method_fns(methods);

    create_lib_rs_file();
}
