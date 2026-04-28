use std::path::Path;
use std::process::exit;
use std::{ fs::{ File, write }, io::BufReader };
use serde::de::DeserializeOwned;

// Iclude property models to map the json
include!("./src/models/common.rs");
include!("./src/models/property.rs");
include!("./src/models/method.rs");

// Exact error is not known at compile time because std::fs::File &
// serde_json::from_reader have different errors
// but both implement the std::error::Error trait
// therefore dynamic dispatch at runtime to decide the type
type ApiFilesResult<T> = Result<Vec<T>, Box<dyn std::error::Error>>;

///////////////////////////////////////////////////////////////////////////
/// COMMON ////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////
const HEADER: &str = "
// Copyright (c) 2026 Efstratios Pahis
// SPDX-License-Identifier: MPL-2.0
";

const COMMON_INIT_CODE: &str =
    "
#![allow(unused_variables)]
#![allow(non_snake_case)]

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct APICallResponse {
    jsonrpc: String,
    result: String,
    id: String,
}

type APICallResult = Result<APICallResponse, Box<dyn std::error::Error>>;\n
";

/// Read json by passed file_path
fn read_json_file<T>(file_path: &str) -> ApiFilesResult<T> where T: DeserializeOwned {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let json_file: Vec<T> = serde_json::from_reader(reader)?;

    Ok(json_file)
}

fn write_to_file(code: String, file_path: &str) {
    let dest_path = Path::new(file_path);

    write(&dest_path, code).unwrap();
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

///////////////////////////////////////////////////////////////////////////
/// Barco Property API ////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////
fn create_property_fns(properties: Vec<Property>) {
    // boilerplate
    let mut generated_code = String::from(HEADER);
    // init code
    generated_code.push_str(COMMON_INIT_CODE);
    // Iterate over all properties
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
        String::from("get_") + &property_name + "(address: &str)"
    } else {
        String::from("set_") +
            &property_name +
            format!(
                "(\n    address: &str,\n    value: {})",
                match_set_value_type(&property.set, "value").unwrap()
            ).as_str()
    }
}

fn get_jsonrpc_generated_properties_code(property: &Property) -> String {
    // Helper func to replace id
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
            "///{}
pub async fn {} -> APICallResult {{
    let client = reqwest::Client::new();
    let payload = serde_json::json!({{\"jsonrpc\":\"{}\",\"method\":\"{}\",\"id\":\"{}\",\"params\":{{\"property\":\"{}\",\"value\":value}}}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}}\n",
            &property.description,
            fn_signature,
            payload.jsonrpc,
            payload.method,
            payload.id,
            payload.params.get("property").unwrap()
        )
    } else {
        format!(
            "///{}
pub async fn {} -> APICallResult {{
    let client = reqwest::Client::new();
    let res = client.post(address).body({:?}).send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}}\n",
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
    // boilerplate
    let mut generated_code = String::from(HEADER);
    // init code
    generated_code.push_str(COMMON_INIT_CODE);
    // generic_call
    generated_code.push_str(
        "/// custom pulse method api call
pub async fn custom_method_call(address: &str, method_name: &str, params: std::collections::HashMap<String, String>) -> APICallResult {{
let client = reqwest::Client::new();
    let payload = serde_json::json!({\"jsonrpc\":2.0,\"method\":method_name,\"id\":method_name,\"params\":params}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}}\n"
    );
    // Iterate over all properties
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
        // Common
        let param_value_type = match_set_value_type(&Some(method.request.clone()), param_key);
        if param_value_type.is_some() {
            // Signature
            param_signature_tmp += &format!("{}: {},", param_key, param_value_type.unwrap());
            // Body
            body_params_tmp += &format!("\"{}\": {},", param_key, param_key);
        }
        // Signature
        param_signature = Some(param_signature_tmp.clone());
        // Body
        body_params = Some(body_params_tmp.clone());
    });

    if param_signature.is_some() {
        (
            String::from("set_") +
                &method_name +
                format!("(\n    address: &str,\n    {})", param_signature.unwrap()).as_str(),
            body_params,
        )
    } else {
        (String::from("set_") + &method_name + "(address: &str)", body_params)
    }
}

fn get_jsonrpc_generated_methods_code(method: &Method) -> String {
    // Helper func to replace id
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
            "///{}
pub async fn {} -> APICallResult {{
    let client = reqwest::Client::new();
    let payload = serde_json::json!({{\"jsonrpc\":\"{}\",\"method\":\"{}\",\"id\":\"{}\",\"params\":{{{}}}}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}}\n",
            &method.description,
            fn_signature,
            payload.jsonrpc,
            payload.method,
            payload.id,
            fn_body.as_ref().unwrap()
        )
    } else {
        format!(
            "///{}
pub async fn {} -> APICallResult {{
    let client = reqwest::Client::new();
    let res = client.post(address).body({:?}).send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}}\n",
            &method.description,
            fn_signature,
            serde_json::to_string(&payload).unwrap()
        )
    }
}

fn main() {
    let file_path_properties = "./src/barco_api/properties.json";
    let properties: Vec<Property> = read_json_file(file_path_properties).unwrap_or_else(|err| {
        println!("{:?}", err);
        exit(1);
    });
    create_property_fns(properties);

    let file_path_methods = "./src/barco_api/methods.json";
    let methods: Vec<Method> = read_json_file(file_path_methods).unwrap_or_else(|err| {
        println!("{:?}", err);
        exit(1);
    });
    create_method_fns(methods);
}
