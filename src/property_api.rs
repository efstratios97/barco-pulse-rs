
// Copyright (c) 2026 Efstratios Pahis
// SPDX-License-Identifier: MPL-2.0

#![allow(unused_variables)]
#![allow(non_snake_case)]

#[derive(serde::Deserialize, serde::Serialize)]
pub struct APICallResponse {
    jsonrpc: String,
    result: String,
    id: String,
}

type APICallResult = Result<APICallResponse, Box<dyn std::error::Error>>;

///The error message in case of a backup error state.
pub async fn get_backup_auto_error(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"backup.auto.error\",\"params\":{\"property\":\"backup.auto.error\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Initial IP address for backup actions.
pub async fn get_backup_auto_ipaddress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"backup.auto.ipaddress\",\"params\":{\"property\":\"backup.auto.ipaddress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Date on which SyncState became Asynchronous
pub async fn get_backup_auto_outofsyncdate(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"backup.auto.outofsyncdate\",\"params\":{\"property\":\"backup.auto.outofsyncdate\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The current state of the automatic backup
pub async fn get_backup_auto_state(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"backup.auto.state\",\"params\":{\"property\":\"backup.auto.state\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Whether the backup is in sync
pub async fn get_backup_auto_syncstate(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"backup.auto.syncstate\",\"params\":{\"property\":\"backup.auto.syncstate\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Progress for the selftest.
pub async fn get_diagnostics_selftest_progress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"diagnostics.selftest.progress\",\"params\":{\"property\":\"diagnostics.selftest.progress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The result and overview of all executed selftests
pub async fn get_diagnostics_selftest_result(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"diagnostics.selftest.result\",\"params\":{\"property\":\"diagnostics.selftest.result\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The current status of the executed test-list
pub async fn get_diagnostics_selftest_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"diagnostics.selftest.status\",\"params\":{\"property\":\"diagnostics.selftest.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Artnet enabled or not
pub async fn set_dmx_artnet(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"dmx.artnet","params":{"property":"dmx.artnet","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Artnet net selection
pub async fn set_dmx_artnetnet(
    address: &str,
    value: u64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"dmx.artnetnet","params":{"property":"dmx.artnetnet","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Artnet universe selection
pub async fn set_dmx_artnetuniverse(
    address: &str,
    value: u64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"dmx.artnetuniverse","params":{"property":"dmx.artnetuniverse","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Current mode
pub async fn set_dmx_mode(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"dmx.mode","params":{"property":"dmx.mode","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Description for the dmx channel
pub async fn get_dmx_monitor_channel01_function(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"dmx.monitor.channel01.function\",\"params\":{\"property\":\"dmx.monitor.channel01.function\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Offset of the channel.
pub async fn get_dmx_monitor_channel01_offset(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"dmx.monitor.channel01.offset\",\"params\":{\"property\":\"dmx.monitor.channel01.offset\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Current dmx value for the channel
pub async fn get_dmx_monitor_channel01_value(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"dmx.monitor.channel01.value\",\"params\":{\"property\":\"dmx.monitor.channel01.value\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description for the dmx channel
pub async fn get_dmx_monitor_channel02_function(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"dmx.monitor.channel02.function\",\"params\":{\"property\":\"dmx.monitor.channel02.function\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Offset of the channel.
pub async fn get_dmx_monitor_channel02_offset(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"dmx.monitor.channel02.offset\",\"params\":{\"property\":\"dmx.monitor.channel02.offset\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Current dmx value for the channel
pub async fn get_dmx_monitor_channel02_value(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"dmx.monitor.channel02.value\",\"params\":{\"property\":\"dmx.monitor.channel02.value\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///true indicates that a dmx (if artnet setting is deactivated) or artnet package (if artnet setting is active) was received in the last 10 seconds.
pub async fn get_dmx_monitor_connectionstate_active(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"dmx.monitor.connectionstate.active\",\"params\":{\"property\":\"dmx.monitor.connectionstate.active\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Shutdown enabled or not
pub async fn set_dmx_shutdown(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"dmx.shutdown","params":{"property":"dmx.shutdown","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Time out for shutdown in minutes.
pub async fn set_dmx_shutdowntimeout(
    address: &str,
    value: u64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"dmx.shutdowntimeout","params":{"property":"dmx.shutdowntimeout","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The dmx start channel [1..512].
pub async fn set_dmx_startchannel(
    address: &str,
    value: u64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"dmx.startchannel","params":{"property":"dmx.startchannel","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///[DEPRECATED] Replaced by 'system.health'
pub async fn get_environment_alarmstate(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"environment.alarmstate\",\"params\":{\"property\":\"environment.alarmstate\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The firmware packages available for upgrade
pub async fn get_firmware_availablepackages(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.availablepackages\",\"params\":{\"property\":\"firmware.availablepackages\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_backplane_boot_redstar_actualversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.backplane-boot-redstar.actualversion\",\"params\":{\"property\":\"firmware.component.backplane-boot-redstar.actualversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_backplane_boot_redstar_displayname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.backplane-boot-redstar.displayname\",\"params\":{\"property\":\"firmware.component.backplane-boot-redstar.displayname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_backplane_boot_redstar_error(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.backplane-boot-redstar.error\",\"params\":{\"property\":\"firmware.component.backplane-boot-redstar.error\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_backplane_boot_redstar_mismatch(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.backplane-boot-redstar.mismatch\",\"params\":{\"property\":\"firmware.component.backplane-boot-redstar.mismatch\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_backplane_boot_redstar_progress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.backplane-boot-redstar.progress\",\"params\":{\"property\":\"firmware.component.backplane-boot-redstar.progress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_backplane_boot_redstar_requiredversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.backplane-boot-redstar.requiredversion\",\"params\":{\"property\":\"firmware.component.backplane-boot-redstar.requiredversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_backplane_boot_redstar_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.backplane-boot-redstar.status\",\"params\":{\"property\":\"firmware.component.backplane-boot-redstar.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_backplane_redstar_fpga_actualversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.backplane-redstar-fpga.actualversion\",\"params\":{\"property\":\"firmware.component.backplane-redstar-fpga.actualversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_backplane_redstar_fpga_displayname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.backplane-redstar-fpga.displayname\",\"params\":{\"property\":\"firmware.component.backplane-redstar-fpga.displayname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_backplane_redstar_fpga_error(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.backplane-redstar-fpga.error\",\"params\":{\"property\":\"firmware.component.backplane-redstar-fpga.error\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_backplane_redstar_fpga_mismatch(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.backplane-redstar-fpga.mismatch\",\"params\":{\"property\":\"firmware.component.backplane-redstar-fpga.mismatch\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_backplane_redstar_fpga_progress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.backplane-redstar-fpga.progress\",\"params\":{\"property\":\"firmware.component.backplane-redstar-fpga.progress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_backplane_redstar_fpga_requiredversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.backplane-redstar-fpga.requiredversion\",\"params\":{\"property\":\"firmware.component.backplane-redstar-fpga.requiredversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_backplane_redstar_fpga_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.backplane-redstar-fpga.status\",\"params\":{\"property\":\"firmware.component.backplane-redstar-fpga.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_backplane_run_redstar_actualversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.backplane-run-redstar.actualversion\",\"params\":{\"property\":\"firmware.component.backplane-run-redstar.actualversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_backplane_run_redstar_displayname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.backplane-run-redstar.displayname\",\"params\":{\"property\":\"firmware.component.backplane-run-redstar.displayname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_backplane_run_redstar_error(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.backplane-run-redstar.error\",\"params\":{\"property\":\"firmware.component.backplane-run-redstar.error\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_backplane_run_redstar_mismatch(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.backplane-run-redstar.mismatch\",\"params\":{\"property\":\"firmware.component.backplane-run-redstar.mismatch\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_backplane_run_redstar_progress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.backplane-run-redstar.progress\",\"params\":{\"property\":\"firmware.component.backplane-run-redstar.progress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_backplane_run_redstar_requiredversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.backplane-run-redstar.requiredversion\",\"params\":{\"property\":\"firmware.component.backplane-run-redstar.requiredversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_backplane_run_redstar_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.backplane-run-redstar.status\",\"params\":{\"property\":\"firmware.component.backplane-run-redstar.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///List of components
pub async fn get_firmware_component_components(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.components\",\"params\":{\"property\":\"firmware.component.components\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_formatter_1dlp_fpga_actualversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-1dlp-fpga.actualversion\",\"params\":{\"property\":\"firmware.component.formatter-1dlp-fpga.actualversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_formatter_1dlp_fpga_displayname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-1dlp-fpga.displayname\",\"params\":{\"property\":\"firmware.component.formatter-1dlp-fpga.displayname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_formatter_1dlp_fpga_error(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-1dlp-fpga.error\",\"params\":{\"property\":\"firmware.component.formatter-1dlp-fpga.error\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_formatter_1dlp_fpga_mismatch(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-1dlp-fpga.mismatch\",\"params\":{\"property\":\"firmware.component.formatter-1dlp-fpga.mismatch\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_formatter_1dlp_fpga_progress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-1dlp-fpga.progress\",\"params\":{\"property\":\"firmware.component.formatter-1dlp-fpga.progress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_formatter_1dlp_fpga_requiredversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-1dlp-fpga.requiredversion\",\"params\":{\"property\":\"firmware.component.formatter-1dlp-fpga.requiredversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_formatter_1dlp_fpga_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-1dlp-fpga.status\",\"params\":{\"property\":\"firmware.component.formatter-1dlp-fpga.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_formatter_ddp442x_sequences_actualversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-ddp442x-sequences.actualversion\",\"params\":{\"property\":\"firmware.component.formatter-ddp442x-sequences.actualversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_formatter_ddp442x_sequences_displayname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-ddp442x-sequences.displayname\",\"params\":{\"property\":\"firmware.component.formatter-ddp442x-sequences.displayname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_formatter_ddp442x_sequences_error(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-ddp442x-sequences.error\",\"params\":{\"property\":\"firmware.component.formatter-ddp442x-sequences.error\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_formatter_ddp442x_sequences_mismatch(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-ddp442x-sequences.mismatch\",\"params\":{\"property\":\"firmware.component.formatter-ddp442x-sequences.mismatch\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_formatter_ddp442x_sequences_progress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-ddp442x-sequences.progress\",\"params\":{\"property\":\"firmware.component.formatter-ddp442x-sequences.progress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_formatter_ddp442x_sequences_requiredversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-ddp442x-sequences.requiredversion\",\"params\":{\"property\":\"firmware.component.formatter-ddp442x-sequences.requiredversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_formatter_ddp442x_sequences_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-ddp442x-sequences.status\",\"params\":{\"property\":\"firmware.component.formatter-ddp442x-sequences.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_formatter_rallar_boot_actualversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-rallar-boot.actualversion\",\"params\":{\"property\":\"firmware.component.formatter-rallar-boot.actualversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_formatter_rallar_boot_displayname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-rallar-boot.displayname\",\"params\":{\"property\":\"firmware.component.formatter-rallar-boot.displayname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_formatter_rallar_boot_error(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-rallar-boot.error\",\"params\":{\"property\":\"firmware.component.formatter-rallar-boot.error\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_formatter_rallar_boot_mismatch(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-rallar-boot.mismatch\",\"params\":{\"property\":\"firmware.component.formatter-rallar-boot.mismatch\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_formatter_rallar_boot_progress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-rallar-boot.progress\",\"params\":{\"property\":\"firmware.component.formatter-rallar-boot.progress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_formatter_rallar_boot_requiredversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-rallar-boot.requiredversion\",\"params\":{\"property\":\"firmware.component.formatter-rallar-boot.requiredversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_formatter_rallar_boot_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-rallar-boot.status\",\"params\":{\"property\":\"firmware.component.formatter-rallar-boot.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_formatter_rallar_run_actualversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-rallar-run.actualversion\",\"params\":{\"property\":\"firmware.component.formatter-rallar-run.actualversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_formatter_rallar_run_displayname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-rallar-run.displayname\",\"params\":{\"property\":\"firmware.component.formatter-rallar-run.displayname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_formatter_rallar_run_error(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-rallar-run.error\",\"params\":{\"property\":\"firmware.component.formatter-rallar-run.error\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_formatter_rallar_run_mismatch(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-rallar-run.mismatch\",\"params\":{\"property\":\"firmware.component.formatter-rallar-run.mismatch\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_formatter_rallar_run_progress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-rallar-run.progress\",\"params\":{\"property\":\"firmware.component.formatter-rallar-run.progress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_formatter_rallar_run_requiredversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-rallar-run.requiredversion\",\"params\":{\"property\":\"firmware.component.formatter-rallar-run.requiredversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_formatter_rallar_run_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.formatter-rallar-run.status\",\"params\":{\"property\":\"firmware.component.formatter-rallar-run.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_green_controller_boot_v1_actualversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.green-controller-boot-v1.actualversion\",\"params\":{\"property\":\"firmware.component.green-controller-boot-v1.actualversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_green_controller_boot_v1_displayname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.green-controller-boot-v1.displayname\",\"params\":{\"property\":\"firmware.component.green-controller-boot-v1.displayname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_green_controller_boot_v1_error(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.green-controller-boot-v1.error\",\"params\":{\"property\":\"firmware.component.green-controller-boot-v1.error\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_green_controller_boot_v1_mismatch(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.green-controller-boot-v1.mismatch\",\"params\":{\"property\":\"firmware.component.green-controller-boot-v1.mismatch\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_green_controller_boot_v1_progress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.green-controller-boot-v1.progress\",\"params\":{\"property\":\"firmware.component.green-controller-boot-v1.progress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_green_controller_boot_v1_requiredversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.green-controller-boot-v1.requiredversion\",\"params\":{\"property\":\"firmware.component.green-controller-boot-v1.requiredversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_green_controller_boot_v1_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.green-controller-boot-v1.status\",\"params\":{\"property\":\"firmware.component.green-controller-boot-v1.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_green_controller_run_v1_actualversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.green-controller-run-v1.actualversion\",\"params\":{\"property\":\"firmware.component.green-controller-run-v1.actualversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_green_controller_run_v1_displayname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.green-controller-run-v1.displayname\",\"params\":{\"property\":\"firmware.component.green-controller-run-v1.displayname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_green_controller_run_v1_error(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.green-controller-run-v1.error\",\"params\":{\"property\":\"firmware.component.green-controller-run-v1.error\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_green_controller_run_v1_mismatch(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.green-controller-run-v1.mismatch\",\"params\":{\"property\":\"firmware.component.green-controller-run-v1.mismatch\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_green_controller_run_v1_progress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.green-controller-run-v1.progress\",\"params\":{\"property\":\"firmware.component.green-controller-run-v1.progress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_green_controller_run_v1_requiredversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.green-controller-run-v1.requiredversion\",\"params\":{\"property\":\"firmware.component.green-controller-run-v1.requiredversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_green_controller_run_v1_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.green-controller-run-v1.status\",\"params\":{\"property\":\"firmware.component.green-controller-run-v1.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_ld1_megmeet_psu_actualversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.ld1-megmeet-psu.actualversion\",\"params\":{\"property\":\"firmware.component.ld1-megmeet-psu.actualversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_ld1_megmeet_psu_displayname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.ld1-megmeet-psu.displayname\",\"params\":{\"property\":\"firmware.component.ld1-megmeet-psu.displayname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_ld1_megmeet_psu_error(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.ld1-megmeet-psu.error\",\"params\":{\"property\":\"firmware.component.ld1-megmeet-psu.error\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_ld1_megmeet_psu_mismatch(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.ld1-megmeet-psu.mismatch\",\"params\":{\"property\":\"firmware.component.ld1-megmeet-psu.mismatch\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_ld1_megmeet_psu_progress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.ld1-megmeet-psu.progress\",\"params\":{\"property\":\"firmware.component.ld1-megmeet-psu.progress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_ld1_megmeet_psu_requiredversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.ld1-megmeet-psu.requiredversion\",\"params\":{\"property\":\"firmware.component.ld1-megmeet-psu.requiredversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_ld1_megmeet_psu_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.ld1-megmeet-psu.status\",\"params\":{\"property\":\"firmware.component.ld1-megmeet-psu.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_ld2_megmeet_psu_actualversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.ld2-megmeet-psu.actualversion\",\"params\":{\"property\":\"firmware.component.ld2-megmeet-psu.actualversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_ld2_megmeet_psu_displayname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.ld2-megmeet-psu.displayname\",\"params\":{\"property\":\"firmware.component.ld2-megmeet-psu.displayname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_ld2_megmeet_psu_error(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.ld2-megmeet-psu.error\",\"params\":{\"property\":\"firmware.component.ld2-megmeet-psu.error\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_ld2_megmeet_psu_mismatch(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.ld2-megmeet-psu.mismatch\",\"params\":{\"property\":\"firmware.component.ld2-megmeet-psu.mismatch\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_ld2_megmeet_psu_progress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.ld2-megmeet-psu.progress\",\"params\":{\"property\":\"firmware.component.ld2-megmeet-psu.progress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_ld2_megmeet_psu_requiredversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.ld2-megmeet-psu.requiredversion\",\"params\":{\"property\":\"firmware.component.ld2-megmeet-psu.requiredversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_ld2_megmeet_psu_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.ld2-megmeet-psu.status\",\"params\":{\"property\":\"firmware.component.ld2-megmeet-psu.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_linux_actualversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.linux.actualversion\",\"params\":{\"property\":\"firmware.component.linux.actualversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_linux_displayname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.linux.displayname\",\"params\":{\"property\":\"firmware.component.linux.displayname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_linux_error(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.linux.error\",\"params\":{\"property\":\"firmware.component.linux.error\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_linux_mismatch(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.linux.mismatch\",\"params\":{\"property\":\"firmware.component.linux.mismatch\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_linux_progress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.linux.progress\",\"params\":{\"property\":\"firmware.component.linux.progress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_linux_requiredversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.linux.requiredversion\",\"params\":{\"property\":\"firmware.component.linux.requiredversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_linux_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.linux.status\",\"params\":{\"property\":\"firmware.component.linux.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_boot_actualversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.mysticum-image-processing-3-mk22-boot.actualversion\",\"params\":{\"property\":\"firmware.component.mysticum-image-processing-3-mk22-boot.actualversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_boot_displayname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.mysticum-image-processing-3-mk22-boot.displayname\",\"params\":{\"property\":\"firmware.component.mysticum-image-processing-3-mk22-boot.displayname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_boot_error(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.mysticum-image-processing-3-mk22-boot.error\",\"params\":{\"property\":\"firmware.component.mysticum-image-processing-3-mk22-boot.error\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_boot_mismatch(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.mysticum-image-processing-3-mk22-boot.mismatch\",\"params\":{\"property\":\"firmware.component.mysticum-image-processing-3-mk22-boot.mismatch\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_boot_progress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.mysticum-image-processing-3-mk22-boot.progress\",\"params\":{\"property\":\"firmware.component.mysticum-image-processing-3-mk22-boot.progress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_boot_requiredversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.mysticum-image-processing-3-mk22-boot.requiredversion\",\"params\":{\"property\":\"firmware.component.mysticum-image-processing-3-mk22-boot.requiredversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_boot_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.mysticum-image-processing-3-mk22-boot.status\",\"params\":{\"property\":\"firmware.component.mysticum-image-processing-3-mk22-boot.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_fpga_actualversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.mysticum-image-processing-3-mk22-fpga.actualversion\",\"params\":{\"property\":\"firmware.component.mysticum-image-processing-3-mk22-fpga.actualversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_fpga_displayname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.mysticum-image-processing-3-mk22-fpga.displayname\",\"params\":{\"property\":\"firmware.component.mysticum-image-processing-3-mk22-fpga.displayname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_fpga_error(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.mysticum-image-processing-3-mk22-fpga.error\",\"params\":{\"property\":\"firmware.component.mysticum-image-processing-3-mk22-fpga.error\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_fpga_mismatch(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.mysticum-image-processing-3-mk22-fpga.mismatch\",\"params\":{\"property\":\"firmware.component.mysticum-image-processing-3-mk22-fpga.mismatch\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_fpga_progress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.mysticum-image-processing-3-mk22-fpga.progress\",\"params\":{\"property\":\"firmware.component.mysticum-image-processing-3-mk22-fpga.progress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_fpga_requiredversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.mysticum-image-processing-3-mk22-fpga.requiredversion\",\"params\":{\"property\":\"firmware.component.mysticum-image-processing-3-mk22-fpga.requiredversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_fpga_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.mysticum-image-processing-3-mk22-fpga.status\",\"params\":{\"property\":\"firmware.component.mysticum-image-processing-3-mk22-fpga.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_run_actualversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.mysticum-image-processing-3-mk22-run.actualversion\",\"params\":{\"property\":\"firmware.component.mysticum-image-processing-3-mk22-run.actualversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_run_displayname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.mysticum-image-processing-3-mk22-run.displayname\",\"params\":{\"property\":\"firmware.component.mysticum-image-processing-3-mk22-run.displayname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_run_error(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.mysticum-image-processing-3-mk22-run.error\",\"params\":{\"property\":\"firmware.component.mysticum-image-processing-3-mk22-run.error\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_run_mismatch(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.mysticum-image-processing-3-mk22-run.mismatch\",\"params\":{\"property\":\"firmware.component.mysticum-image-processing-3-mk22-run.mismatch\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_run_progress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.mysticum-image-processing-3-mk22-run.progress\",\"params\":{\"property\":\"firmware.component.mysticum-image-processing-3-mk22-run.progress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_run_requiredversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.mysticum-image-processing-3-mk22-run.requiredversion\",\"params\":{\"property\":\"firmware.component.mysticum-image-processing-3-mk22-run.requiredversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_run_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.mysticum-image-processing-3-mk22-run.status\",\"params\":{\"property\":\"firmware.component.mysticum-image-processing-3-mk22-run.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_redstar_fan_motor_control_board_fpga_actualversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.redstar-fan-motor-control-board-fpga.actualversion\",\"params\":{\"property\":\"firmware.component.redstar-fan-motor-control-board-fpga.actualversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_redstar_fan_motor_control_board_fpga_displayname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.redstar-fan-motor-control-board-fpga.displayname\",\"params\":{\"property\":\"firmware.component.redstar-fan-motor-control-board-fpga.displayname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_redstar_fan_motor_control_board_fpga_error(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.redstar-fan-motor-control-board-fpga.error\",\"params\":{\"property\":\"firmware.component.redstar-fan-motor-control-board-fpga.error\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_redstar_fan_motor_control_board_fpga_mismatch(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.redstar-fan-motor-control-board-fpga.mismatch\",\"params\":{\"property\":\"firmware.component.redstar-fan-motor-control-board-fpga.mismatch\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_redstar_fan_motor_control_board_fpga_progress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.redstar-fan-motor-control-board-fpga.progress\",\"params\":{\"property\":\"firmware.component.redstar-fan-motor-control-board-fpga.progress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_redstar_fan_motor_control_board_fpga_requiredversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.redstar-fan-motor-control-board-fpga.requiredversion\",\"params\":{\"property\":\"firmware.component.redstar-fan-motor-control-board-fpga.requiredversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_redstar_fan_motor_control_board_fpga_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.redstar-fan-motor-control-board-fpga.status\",\"params\":{\"property\":\"firmware.component.redstar-fan-motor-control-board-fpga.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_redstar_fan_motor_control_boot_actualversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.redstar-fan-motor-control-boot.actualversion\",\"params\":{\"property\":\"firmware.component.redstar-fan-motor-control-boot.actualversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_redstar_fan_motor_control_boot_displayname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.redstar-fan-motor-control-boot.displayname\",\"params\":{\"property\":\"firmware.component.redstar-fan-motor-control-boot.displayname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_redstar_fan_motor_control_boot_error(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.redstar-fan-motor-control-boot.error\",\"params\":{\"property\":\"firmware.component.redstar-fan-motor-control-boot.error\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_redstar_fan_motor_control_boot_mismatch(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.redstar-fan-motor-control-boot.mismatch\",\"params\":{\"property\":\"firmware.component.redstar-fan-motor-control-boot.mismatch\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_redstar_fan_motor_control_boot_progress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.redstar-fan-motor-control-boot.progress\",\"params\":{\"property\":\"firmware.component.redstar-fan-motor-control-boot.progress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_redstar_fan_motor_control_boot_requiredversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.redstar-fan-motor-control-boot.requiredversion\",\"params\":{\"property\":\"firmware.component.redstar-fan-motor-control-boot.requiredversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_redstar_fan_motor_control_boot_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.redstar-fan-motor-control-boot.status\",\"params\":{\"property\":\"firmware.component.redstar-fan-motor-control-boot.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_redstar_fan_motor_control_run_actualversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.redstar-fan-motor-control-run.actualversion\",\"params\":{\"property\":\"firmware.component.redstar-fan-motor-control-run.actualversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_redstar_fan_motor_control_run_displayname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.redstar-fan-motor-control-run.displayname\",\"params\":{\"property\":\"firmware.component.redstar-fan-motor-control-run.displayname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_redstar_fan_motor_control_run_error(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.redstar-fan-motor-control-run.error\",\"params\":{\"property\":\"firmware.component.redstar-fan-motor-control-run.error\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_redstar_fan_motor_control_run_mismatch(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.redstar-fan-motor-control-run.mismatch\",\"params\":{\"property\":\"firmware.component.redstar-fan-motor-control-run.mismatch\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_redstar_fan_motor_control_run_progress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.redstar-fan-motor-control-run.progress\",\"params\":{\"property\":\"firmware.component.redstar-fan-motor-control-run.progress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_redstar_fan_motor_control_run_requiredversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.redstar-fan-motor-control-run.requiredversion\",\"params\":{\"property\":\"firmware.component.redstar-fan-motor-control-run.requiredversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_redstar_fan_motor_control_run_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.redstar-fan-motor-control-run.status\",\"params\":{\"property\":\"firmware.component.redstar-fan-motor-control-run.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The status of the current component firmware upgrade
pub async fn get_firmware_component_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.status\",\"params\":{\"property\":\"firmware.component.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_u_boot_actualversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.u-boot.actualversion\",\"params\":{\"property\":\"firmware.component.u-boot.actualversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_u_boot_displayname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.u-boot.displayname\",\"params\":{\"property\":\"firmware.component.u-boot.displayname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_u_boot_error(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.u-boot.error\",\"params\":{\"property\":\"firmware.component.u-boot.error\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_u_boot_mismatch(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.u-boot.mismatch\",\"params\":{\"property\":\"firmware.component.u-boot.mismatch\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_u_boot_progress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.u-boot.progress\",\"params\":{\"property\":\"firmware.component.u-boot.progress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_u_boot_requiredversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.u-boot.requiredversion\",\"params\":{\"property\":\"firmware.component.u-boot.requiredversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_u_boot_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.u-boot.status\",\"params\":{\"property\":\"firmware.component.u-boot.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_zynq_fpga_actualversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.zynq-fpga.actualversion\",\"params\":{\"property\":\"firmware.component.zynq-fpga.actualversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_zynq_fpga_displayname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.zynq-fpga.displayname\",\"params\":{\"property\":\"firmware.component.zynq-fpga.displayname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_zynq_fpga_error(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.zynq-fpga.error\",\"params\":{\"property\":\"firmware.component.zynq-fpga.error\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_zynq_fpga_mismatch(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.zynq-fpga.mismatch\",\"params\":{\"property\":\"firmware.component.zynq-fpga.mismatch\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_zynq_fpga_progress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.zynq-fpga.progress\",\"params\":{\"property\":\"firmware.component.zynq-fpga.progress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_zynq_fpga_requiredversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.zynq-fpga.requiredversion\",\"params\":{\"property\":\"firmware.component.zynq-fpga.requiredversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_zynq_fpga_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.component.zynq-fpga.status\",\"params\":{\"property\":\"firmware.component.zynq-fpga.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The error of the last firmware upgrade (only set if status is error)
pub async fn get_firmware_error(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.error\",\"params\":{\"property\":\"firmware.error\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Indiates whether or not the user is required to accept the term agreement.
pub async fn get_firmware_eula_requiretermagreement(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.eula.requiretermagreement\",\"params\":{\"property\":\"firmware.eula.requiretermagreement\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The user agreement with the firmware EULA.
pub async fn set_firmware_eula_termagreement(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"firmware.eula.termagreement","params":{"property":"firmware.eula.termagreement","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The version of the currently installed firmware.
pub async fn get_firmware_firmwareversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.firmwareversion\",\"params\":{\"property\":\"firmware.firmwareversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The progress of the current firmware upgrade
pub async fn get_firmware_progress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.progress\",\"params\":{\"property\":\"firmware.progress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The status of the current firmware upgrade
pub async fn get_firmware_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"firmware.status\",\"params\":{\"property\":\"firmware.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description of gpo
pub async fn get_gpio_gpo_trigger1_description(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"gpio.gpo.trigger1.description\",\"params\":{\"property\":\"gpio.gpo.trigger1.description\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Mode of output, level or pulsetrain
pub async fn set_gpio_gpo_trigger1_mode(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"gpio.gpo.trigger1.mode","params":{"property":"gpio.gpo.trigger1.mode","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Name of gpo
pub async fn get_gpio_gpo_trigger1_name(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"gpio.gpo.trigger1.name\",\"params\":{\"property\":\"gpio.gpo.trigger1.name\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Is GPIO reserved
pub async fn get_gpio_gpo_trigger1_reserved(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"gpio.gpo.trigger1.reserved\",\"params\":{\"property\":\"gpio.gpo.trigger1.reserved\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Value of output
pub async fn set_gpio_gpo_trigger1_value(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"gpio.gpo.trigger1.value","params":{"property":"gpio.gpo.trigger1.value","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Description of gpo
pub async fn get_gpio_gpo_trigger2_description(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"gpio.gpo.trigger2.description\",\"params\":{\"property\":\"gpio.gpo.trigger2.description\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Mode of output, level or pulsetrain
pub async fn set_gpio_gpo_trigger2_mode(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"gpio.gpo.trigger2.mode","params":{"property":"gpio.gpo.trigger2.mode","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Name of gpo
pub async fn get_gpio_gpo_trigger2_name(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"gpio.gpo.trigger2.name\",\"params\":{\"property\":\"gpio.gpo.trigger2.name\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Is GPIO reserved
pub async fn get_gpio_gpo_trigger2_reserved(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"gpio.gpo.trigger2.reserved\",\"params\":{\"property\":\"gpio.gpo.trigger2.reserved\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Value of output
pub async fn set_gpio_gpo_trigger2_value(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"gpio.gpo.trigger2.value","params":{"property":"gpio.gpo.trigger2.value","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The current status of the mobile connection.
pub async fn get_gsm_data_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"gsm.data.status\",\"params\":{\"property\":\"gsm.data.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The ICCID that identifies the SIM.
pub async fn get_gsm_iccid(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"gsm.iccid\",\"params\":{\"property\":\"gsm.iccid\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The IMEI that identifies the GSM module.
pub async fn get_gsm_imei(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"gsm.imei\",\"params\":{\"property\":\"gsm.imei\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The signal strength in percent.
pub async fn get_gsm_signalstrength(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"gsm.signalstrength\",\"params\":{\"property\":\"gsm.signalstrength\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Indicates which features are preventing CLO from being enabled
pub async fn get_illumination_clo_allowed(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"illumination.clo.allowed\",\"params\":{\"property\":\"illumination.clo.allowed\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Shows the current availability.
pub async fn get_illumination_clo_availability(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"illumination.clo.availability\",\"params\":{\"property\":\"illumination.clo.availability\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///True if constant light output is enabled, false otherwise
pub async fn set_illumination_clo_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"illumination.clo.enable","params":{"property":"illumination.clo.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The percentage to scale the setpoint by.
pub async fn set_illumination_clo_scale(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"illumination.clo.scale","params":{"property":"illumination.clo.scale","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The target luminosity of the light source
pub async fn set_illumination_clo_setpoint(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"illumination.clo.setpoint","params":{"property":"illumination.clo.setpoint","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///State of the CLO
pub async fn get_illumination_clo_state(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"illumination.clo.state\",\"params\":{\"property\":\"illumination.clo.state\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Enable/disable NoSignal dimming of the power
pub async fn set_illumination_nosignal_dimming_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"illumination.nosignal.dimming.enable","params":{"property":"illumination.nosignal.dimming.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The power limitation (in percent) that the NoSignal dimming will apply
pub async fn get_illumination_nosignal_dimming_power(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"illumination.nosignal.dimming.power\",\"params\":{\"property\":\"illumination.nosignal.dimming.power\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///actual power in percent with limits
pub async fn get_illumination_sources_laser_actualpower(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"illumination.sources.laser.actualpower\",\"params\":{\"property\":\"illumination.sources.laser.actualpower\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Describes whether CLO is controlling this source.
pub async fn get_illumination_sources_laser_controlledbyclo(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"illumination.sources.laser.controlledbyclo\",\"params\":{\"property\":\"illumination.sources.laser.controlledbyclo\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Whether power is currently limited.
pub async fn get_illumination_sources_laser_ispowerlimited(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"illumination.sources.laser.ispowerlimited\",\"params\":{\"property\":\"illumination.sources.laser.ispowerlimited\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///maximum power in percent
pub async fn get_illumination_sources_laser_maxpower(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"illumination.sources.laser.maxpower\",\"params\":{\"property\":\"illumination.sources.laser.maxpower\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///minimum power in percent
pub async fn get_illumination_sources_laser_minpower(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"illumination.sources.laser.minpower\",\"params\":{\"property\":\"illumination.sources.laser.minpower\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///target power in percent
pub async fn set_illumination_sources_laser_power(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"illumination.sources.laser.power","params":{"property":"illumination.sources.laser.power","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Contains the reason of the authority that limited the power.
pub async fn get_illumination_sources_laser_powerlimitreason(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"illumination.sources.laser.powerlimitreason\",\"params\":{\"property\":\"illumination.sources.laser.powerlimitreason\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Power precision this source takes into account, expressed as number of digits to the right of the decimal point.
pub async fn get_illumination_sources_laser_powerprecision(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"illumination.sources.laser.powerprecision\",\"params\":{\"property\":\"illumination.sources.laser.powerprecision\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_illumination_sources_laser_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"illumination.sources.laser.status\",\"params\":{\"property\":\"illumination.sources.laser.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_illumination_sources_laser_timcuring_dmd_cureavailable(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"illumination.sources.laser.timcuring.dmd.cureavailable\",\"params\":{\"property\":\"illumination.sources.laser.timcuring.dmd.cureavailable\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///State of the current or past curing execution expressed in the format of a date, a status and a list of failed plates
pub async fn get_illumination_sources_laser_timcuring_dmd_state(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"illumination.sources.laser.timcuring.dmd.state\",\"params\":{\"property\":\"illumination.sources.laser.timcuring.dmd.state\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The state of light
pub async fn get_illumination_state(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"illumination.state\",\"params\":{\"property\":\"illumination.state\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Image brightness/offset. The value is normalized, 0 is default, 1 is 100% offset.
pub async fn set_image_brightness(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.brightness","params":{"property":"image.brightness","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The current BrilliantColor mode.
pub async fn set_image_brilliantcolor_mode(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.brilliantcolor.mode","params":{"property":"image.brilliantcolor.mode","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Desired blue gain value
pub async fn set_image_color_p7_custom_bluegain(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.bluegain","params":{"property":"image.color.p7.custom.bluegain","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Desired blue luminanace
pub async fn set_image_color_p7_custom_bluelum(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.bluelum","params":{"property":"image.color.p7.custom.bluelum","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Desired blue x in xy-coordinates
pub async fn set_image_color_p7_custom_bluex(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.bluex","params":{"property":"image.color.p7.custom.bluex","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Desired blue y in xy-coordinates
pub async fn set_image_color_p7_custom_bluey(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.bluey","params":{"property":"image.color.p7.custom.bluey","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///true if secondaries should be shown (OSD)
pub async fn get_image_color_p7_custom_cmyreadable(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.custom.cmyreadable\",\"params\":{\"property\":\"image.color.p7.custom.cmyreadable\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///true if secondaries are Writable
pub async fn get_image_color_p7_custom_cmywritable(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.custom.cmywritable\",\"params\":{\"property\":\"image.color.p7.custom.cmywritable\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Desired cyan gain value
pub async fn set_image_color_p7_custom_cyangain(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.cyangain","params":{"property":"image.color.p7.custom.cyangain","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Desired cyan luminanace
pub async fn set_image_color_p7_custom_cyanlum(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.cyanlum","params":{"property":"image.color.p7.custom.cyanlum","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Desired cyan x in xy-coordinates
pub async fn set_image_color_p7_custom_cyanx(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.cyanx","params":{"property":"image.color.p7.custom.cyanx","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Desired cyan y in xy-coordinates
pub async fn set_image_color_p7_custom_cyany(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.cyany","params":{"property":"image.color.p7.custom.cyany","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///true when gains are available
pub async fn get_image_color_p7_custom_gainsavailable(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.custom.gainsavailable\",\"params\":{\"property\":\"image.color.p7.custom.gainsavailable\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Desired green gain value
pub async fn set_image_color_p7_custom_greengain(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.greengain","params":{"property":"image.color.p7.custom.greengain","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Desired green luminanace
pub async fn set_image_color_p7_custom_greenlum(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.greenlum","params":{"property":"image.color.p7.custom.greenlum","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Desired green x in xy-coordinates
pub async fn set_image_color_p7_custom_greenx(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.greenx","params":{"property":"image.color.p7.custom.greenx","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Desired green y in xy-coordinates
pub async fn set_image_color_p7_custom_greeny(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.greeny","params":{"property":"image.color.p7.custom.greeny","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///true if luminances are available
pub async fn get_image_color_p7_custom_luminancesavailable(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.custom.luminancesavailable\",\"params\":{\"property\":\"image.color.p7.custom.luminancesavailable\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Desired magenta gain value
pub async fn set_image_color_p7_custom_magentagain(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.magentagain","params":{"property":"image.color.p7.custom.magentagain","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Desired magenta luminanace
pub async fn set_image_color_p7_custom_magentalum(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.magentalum","params":{"property":"image.color.p7.custom.magentalum","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Desired magenta x in xy-coordinates
pub async fn set_image_color_p7_custom_magentax(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.magentax","params":{"property":"image.color.p7.custom.magentax","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Desired magenta y in xy-coordinates
pub async fn set_image_color_p7_custom_magentay(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.magentay","params":{"property":"image.color.p7.custom.magentay","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Description not provided
pub async fn set_image_color_p7_custom_mode(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.mode","params":{"property":"image.color.p7.custom.mode","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Description not provided
pub async fn get_image_color_p7_custom_modes(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.custom.modes\",\"params\":{\"property\":\"image.color.p7.custom.modes\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Desired red gain value
pub async fn set_image_color_p7_custom_redgain(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.redgain","params":{"property":"image.color.p7.custom.redgain","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Desired red luminanace
pub async fn set_image_color_p7_custom_redlum(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.redlum","params":{"property":"image.color.p7.custom.redlum","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Desired red x in xy-coordinates
pub async fn set_image_color_p7_custom_redx(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.redx","params":{"property":"image.color.p7.custom.redx","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Desired red y in xy-coordinates
pub async fn set_image_color_p7_custom_redy(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.redy","params":{"property":"image.color.p7.custom.redy","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///true when R,G,B,C,M,Y gains are available
pub async fn get_image_color_p7_custom_rgbcmygainsavailable(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.custom.rgbcmygainsavailable\",\"params\":{\"property\":\"image.color.p7.custom.rgbcmygainsavailable\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///true if RGB are Writable (not in WHITE mode)
pub async fn get_image_color_p7_custom_rgbwritable(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.custom.rgbwritable\",\"params\":{\"property\":\"image.color.p7.custom.rgbwritable\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Target color coordinates for the current preset
pub async fn get_image_color_p7_custom_target(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.custom.target\",\"params\":{\"property\":\"image.color.p7.custom.target\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Desired white gain value
pub async fn set_image_color_p7_custom_whitegain(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.whitegain","params":{"property":"image.color.p7.custom.whitegain","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///true when white gain is available
pub async fn get_image_color_p7_custom_whitegainavailable(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.custom.whitegainavailable\",\"params\":{\"property\":\"image.color.p7.custom.whitegainavailable\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Desired white luminanace
pub async fn set_image_color_p7_custom_whitelum(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.whitelum","params":{"property":"image.color.p7.custom.whitelum","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Description not provided
pub async fn set_image_color_p7_custom_whitemode(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.whitemode","params":{"property":"image.color.p7.custom.whitemode","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Desired white point temperature
pub async fn set_image_color_p7_custom_whitetemperature(
    address: &str,
    value: u64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.whitetemperature","params":{"property":"image.color.p7.custom.whitetemperature","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///true if White temperature is available
pub async fn get_image_color_p7_custom_whitetemperatureavailable(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.custom.whitetemperatureavailable\",\"params\":{\"property\":\"image.color.p7.custom.whitetemperatureavailable\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///true if White is Writable
pub async fn get_image_color_p7_custom_whitewritable(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.custom.whitewritable\",\"params\":{\"property\":\"image.color.p7.custom.whitewritable\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Desired white x in xy-coordinates
pub async fn set_image_color_p7_custom_whitex(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.whitex","params":{"property":"image.color.p7.custom.whitex","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Desired white y in xy-coordinates
pub async fn set_image_color_p7_custom_whitey(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.whitey","params":{"property":"image.color.p7.custom.whitey","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Desired yellow gain value
pub async fn set_image_color_p7_custom_yellowgain(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.yellowgain","params":{"property":"image.color.p7.custom.yellowgain","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Desired yellow luminanace
pub async fn set_image_color_p7_custom_yellowlum(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.yellowlum","params":{"property":"image.color.p7.custom.yellowlum","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Desired yellow x in xy-coordinates
pub async fn set_image_color_p7_custom_yellowx(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.yellowx","params":{"property":"image.color.p7.custom.yellowx","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Desired yellow y in xy-coordinates
pub async fn set_image_color_p7_custom_yellowy(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.p7.custom.yellowy","params":{"property":"image.color.p7.custom.yellowy","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Native C1 x in xy-coordinates
pub async fn get_image_color_p7_native_c1(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.native.c1\",\"params\":{\"property\":\"image.color.p7.native.c1\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_image_color_p7_native_c1available(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.native.c1available\",\"params\":{\"property\":\"image.color.p7.native.c1available\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Native C2 x in xy-coordinates
pub async fn get_image_color_p7_native_c2(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.native.c2\",\"params\":{\"property\":\"image.color.p7.native.c2\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_image_color_p7_native_c2available(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.native.c2available\",\"params\":{\"property\":\"image.color.p7.native.c2available\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///list available native sets
pub async fn get_image_color_p7_native_list(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.native.list\",\"params\":{\"property\":\"image.color.p7.native.list\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Native C1 x in xy-coordinates
pub async fn get_image_color_p7_native_normal_c1(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.native.normal.c1\",\"params\":{\"property\":\"image.color.p7.native.normal.c1\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_image_color_p7_native_normal_c1available(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.native.normal.c1available\",\"params\":{\"property\":\"image.color.p7.native.normal.c1available\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Native C2 x in xy-coordinates
pub async fn get_image_color_p7_native_normal_c2(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.native.normal.c2\",\"params\":{\"property\":\"image.color.p7.native.normal.c2\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_image_color_p7_native_normal_c2available(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.native.normal.c2available\",\"params\":{\"property\":\"image.color.p7.native.normal.c2available\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Native red x in xy-coordinates
pub async fn get_image_color_p7_native_normal_rgbw(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.native.normal.rgbw\",\"params\":{\"property\":\"image.color.p7.native.normal.rgbw\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_image_color_p7_native_normal_whiteavailable(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.native.normal.whiteavailable\",\"params\":{\"property\":\"image.color.p7.native.normal.whiteavailable\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Native red x in xy-coordinates
pub async fn get_image_color_p7_native_rgbw(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.native.rgbw\",\"params\":{\"property\":\"image.color.p7.native.rgbw\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_image_color_p7_native_whiteavailable(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.color.p7.native.whiteavailable\",\"params\":{\"property\":\"image.color.p7.native.whiteavailable\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///RGB Mode
pub async fn set_image_color_rgbmode_rgbmode(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.color.rgbmode.rgbmode","params":{"property":"image.color.rgbmode.rgbmode","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Capabilities.
pub async fn get_image_connector_displayport_capabilities(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.connector.displayport.capabilities\",\"params\":{\"property\":\"image.connector.displayport.capabilities\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Override the detected signal color primaries. Set to Auto for automatic control.
pub async fn set_image_connector_displayport_colorprimaries(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.connector.displayport.colorprimaries","params":{"property":"image.connector.displayport.colorprimaries","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///true if Color Primaries is available
pub async fn get_image_connector_displayport_colorprimariesavailable(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.connector.displayport.colorprimariesavailable\",\"params\":{\"property\":\"image.connector.displayport.colorprimariesavailable\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Override the detected signal color space. Set to Auto for automatic control.
pub async fn set_image_connector_displayport_colorspace(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.connector.displayport.colorspace","params":{"property":"image.connector.displayport.colorspace","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The signal information of the currently detected signal. If 'active' is false, there is no detected signal and the rest of the information should be disregarded. is_stereo indicates if stereo_mode is different from none.
pub async fn get_image_connector_displayport_detectedsignal(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.connector.displayport.detectedsignal\",\"params\":{\"property\":\"image.connector.displayport.detectedsignal\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Selected EDID for connector
pub async fn set_image_connector_displayport_edid_selected(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.connector.displayport.edid.selected","params":{"property":"image.connector.displayport.edid.selected","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Name of the source
pub async fn get_image_connector_displayport_name(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.connector.displayport.name\",\"params\":{\"property\":\"image.connector.displayport.name\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Override the detected signal signal range. Set to Auto for automatic control.
pub async fn set_image_connector_displayport_signalrange(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.connector.displayport.signalrange","params":{"property":"image.connector.displayport.signalrange","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Capabilities.
pub async fn get_image_connector_hdmi_capabilities(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.connector.hdmi.capabilities\",\"params\":{\"property\":\"image.connector.hdmi.capabilities\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Override the detected signal color primaries. Set to Auto for automatic control.
pub async fn set_image_connector_hdmi_colorprimaries(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.connector.hdmi.colorprimaries","params":{"property":"image.connector.hdmi.colorprimaries","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///true if Color Primaries is available
pub async fn get_image_connector_hdmi_colorprimariesavailable(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.connector.hdmi.colorprimariesavailable\",\"params\":{\"property\":\"image.connector.hdmi.colorprimariesavailable\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Override the detected signal color space. Set to Auto for automatic control.
pub async fn set_image_connector_hdmi_colorspace(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.connector.hdmi.colorspace","params":{"property":"image.connector.hdmi.colorspace","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The signal information of the currently detected signal. If 'active' is false, there is no detected signal and the rest of the information should be disregarded. is_stereo indicates if stereo_mode is different from none.
pub async fn get_image_connector_hdmi_detectedsignal(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.connector.hdmi.detectedsignal\",\"params\":{\"property\":\"image.connector.hdmi.detectedsignal\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Selected EDID for connector
pub async fn set_image_connector_hdmi_edid_selected(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.connector.hdmi.edid.selected","params":{"property":"image.connector.hdmi.edid.selected","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Name of the source
pub async fn get_image_connector_hdmi_name(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.connector.hdmi.name\",\"params\":{\"property\":\"image.connector.hdmi.name\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Override the detected signal signal range. Set to Auto for automatic control.
pub async fn set_image_connector_hdmi_signalrange(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.connector.hdmi.signalrange","params":{"property":"image.connector.hdmi.signalrange","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Image contrast/gain. The value is normalized, 1 is default.
pub async fn set_image_contrast(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.contrast","params":{"property":"image.contrast","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The desired display mode.
pub async fn set_image_display_desireddisplaymode(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.display.desireddisplaymode","params":{"property":"image.display.desireddisplaymode","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The current display mode.
pub async fn get_image_display_displaymode(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.display.displaymode\",\"params\":{\"property\":\"image.display.displaymode\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The display frequency.
pub async fn get_image_display_frequency(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.display.frequency\",\"params\":{\"property\":\"image.display.frequency\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The display synchronous lock state.
pub async fn get_image_display_synchronouslock(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.display.synchronouslock\",\"params\":{\"property\":\"image.display.synchronouslock\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Amount of dynamic contrast that will be applied [0:4]
pub async fn set_image_dynamiccontrast_level(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.dynamiccontrast.level","params":{"property":"image.dynamiccontrast.level","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Image gamma. Default is 2.2
pub async fn set_image_gamma(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.gamma","params":{"property":"image.gamma","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Override the detected signal gamma type. Set to Auto for automatic control.
pub async fn set_image_gammatype(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.gammatype","params":{"property":"image.gammatype","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Intensity
pub async fn set_image_intensity(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.intensity","params":{"property":"image.intensity","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Description not provided
pub async fn set_image_orientation(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.orientation","params":{"property":"image.orientation","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The mode of orientation as auto detected or a manual fixed setting
pub async fn set_image_orientationmode(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.orientationmode","params":{"property":"image.orientationmode","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Bottom edge.
pub async fn set_image_processing_blacklevel_basicblacklevel_bottom(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.blacklevel.basicblacklevel.bottom","params":{"property":"image.processing.blacklevel.basicblacklevel.bottom","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Description not provided
pub async fn set_image_processing_blacklevel_basicblacklevel_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.blacklevel.basicblacklevel.enable","params":{"property":"image.processing.blacklevel.basicblacklevel.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Left edge.
pub async fn set_image_processing_blacklevel_basicblacklevel_left(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.blacklevel.basicblacklevel.left","params":{"property":"image.processing.blacklevel.basicblacklevel.left","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Change the upper level of the black level adjustment
pub async fn set_image_processing_blacklevel_basicblacklevel_level(
    address: &str,
    value: u64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.blacklevel.basicblacklevel.level","params":{"property":"image.processing.blacklevel.basicblacklevel.level","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Right edge.
pub async fn set_image_processing_blacklevel_basicblacklevel_right(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.blacklevel.basicblacklevel.right","params":{"property":"image.processing.blacklevel.basicblacklevel.right","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Top edge.
pub async fn set_image_processing_blacklevel_basicblacklevel_top(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.blacklevel.basicblacklevel.top","params":{"property":"image.processing.blacklevel.basicblacklevel.top","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The gain blue for black level
pub async fn set_image_processing_blacklevel_bluegain(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.blacklevel.bluegain","params":{"property":"image.processing.blacklevel.bluegain","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enable/Disable black level correction
pub async fn set_image_processing_blacklevel_file_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.blacklevel.file.enable","params":{"property":"image.processing.blacklevel.file.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Currently selected file
pub async fn set_image_processing_blacklevel_file_selected(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.blacklevel.file.selected","params":{"property":"image.processing.blacklevel.file.selected","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The gain green for black level
pub async fn set_image_processing_blacklevel_greengain(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.blacklevel.greengain","params":{"property":"image.processing.blacklevel.greengain","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///When enabled and setting one color, the others will move in the same direction
pub async fn set_image_processing_blacklevel_linkrgb(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.blacklevel.linkrgb","params":{"property":"image.processing.blacklevel.linkrgb","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The gain red for black level
pub async fn set_image_processing_blacklevel_redgain(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.blacklevel.redgain","params":{"property":"image.processing.blacklevel.redgain","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Bottom blend edge.
pub async fn set_image_processing_blend_basicblend_bottom(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.blend.basicblend.bottom","params":{"property":"image.processing.blend.basicblend.bottom","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Description not provided
pub async fn set_image_processing_blend_basicblend_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.blend.basicblend.enable","params":{"property":"image.processing.blend.basicblend.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Left blend edge.
pub async fn set_image_processing_blend_basicblend_left(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.blend.basicblend.left","params":{"property":"image.processing.blend.basicblend.left","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Right blend edge.
pub async fn set_image_processing_blend_basicblend_right(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.blend.basicblend.right","params":{"property":"image.processing.blend.basicblend.right","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Top blend edge.
pub async fn set_image_processing_blend_basicblend_top(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.blend.basicblend.top","params":{"property":"image.processing.blend.basicblend.top","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enable/Disable file blend
pub async fn set_image_processing_blend_file_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.blend.file.enable","params":{"property":"image.processing.blend.file.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Max number of selected files
pub async fn get_image_processing_blend_file_maxselected(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.processing.blend.file.maxselected\",\"params\":{\"property\":\"image.processing.blend.file.maxselected\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Currently selected files
pub async fn set_image_processing_blend_file_selected(
    address: &str,
    value: std::vec::Vec<String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.blend.file.selected","params":{"property":"image.processing.blend.file.selected","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///S-Curve exponent strength.
pub async fn set_image_processing_blend_scurve(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.blend.scurve","params":{"property":"image.processing.blend.scurve","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enable/Disable tilted brightness
pub async fn set_image_processing_tiltedbrightness_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.tiltedbrightness.enable","params":{"property":"image.processing.tiltedbrightness.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///horizontal brightness correction
pub async fn set_image_processing_tiltedbrightness_horizontalcorrection(
    address: &str,
    value: u64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.tiltedbrightness.horizontalcorrection","params":{"property":"image.processing.tiltedbrightness.horizontalcorrection","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///vertical brightness correction
pub async fn set_image_processing_tiltedbrightness_verticalcorrection(
    address: &str,
    value: u64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.tiltedbrightness.verticalcorrection","params":{"property":"image.processing.tiltedbrightness.verticalcorrection","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Actual transport delay.
pub async fn get_image_processing_transportdelay_actual(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.processing.transportdelay.actual\",\"params\":{\"property\":\"image.processing.transportdelay.actual\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Desired transport delay.
pub async fn set_image_processing_transportdelay_desired(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.transportdelay.desired","params":{"property":"image.processing.transportdelay.desired","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Minimum transport delay.
pub async fn get_image_processing_transportdelay_minimum(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.processing.transportdelay.minimum\",\"params\":{\"property\":\"image.processing.transportdelay.minimum\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///U vector for bottom left corner. Positive angle is outwards.
pub async fn set_image_processing_warp_bow_bottomleftu(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.bow.bottomleftu","params":{"property":"image.processing.warp.bow.bottomleftu","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///V vector for bottom left corner. Positive angle is outwards.
pub async fn set_image_processing_warp_bow_bottomleftv(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.bow.bottomleftv","params":{"property":"image.processing.warp.bow.bottomleftv","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///U vector for bottom right corner. Positive angle is outwards.
pub async fn set_image_processing_warp_bow_bottomrightu(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.bow.bottomrightu","params":{"property":"image.processing.warp.bow.bottomrightu","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///V vector for bottom right corner. Positive angle is outwards.
pub async fn set_image_processing_warp_bow_bottomrightv(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.bow.bottomrightv","params":{"property":"image.processing.warp.bow.bottomrightv","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enable/Disable bow warp
pub async fn set_image_processing_warp_bow_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.bow.enable","params":{"property":"image.processing.warp.bow.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enable/Disable symmetric mode.
pub async fn set_image_processing_warp_bow_symmetric(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.bow.symmetric","params":{"property":"image.processing.warp.bow.symmetric","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///U vector for top left corner. Positive angle is outwards.
pub async fn set_image_processing_warp_bow_topleftu(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.bow.topleftu","params":{"property":"image.processing.warp.bow.topleftu","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///V vector for top left corner. Positive angle is outwards.
pub async fn set_image_processing_warp_bow_topleftv(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.bow.topleftv","params":{"property":"image.processing.warp.bow.topleftv","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///U vector for top right corner. Positive angle is outwards.
pub async fn set_image_processing_warp_bow_toprightu(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.bow.toprightu","params":{"property":"image.processing.warp.bow.toprightu","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///V vector for top right corner. Positive angle is outwards.
pub async fn set_image_processing_warp_bow_toprightv(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.bow.toprightv","params":{"property":"image.processing.warp.bow.toprightv","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enable/Disable all warp functions
pub async fn set_image_processing_warp_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.enable","params":{"property":"image.processing.warp.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enable/Disable file warp
pub async fn set_image_processing_warp_file_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.file.enable","params":{"property":"image.processing.warp.file.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Currently selected file
pub async fn set_image_processing_warp_file_selected(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.file.selected","params":{"property":"image.processing.warp.file.selected","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Bottom left coordinate in output resolution. Negative values alowed to go outside displayed area.
pub async fn set_image_processing_warp_fourcorners_bottomleft(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.fourcorners.bottomleft","params":{"property":"image.processing.warp.fourcorners.bottomleft","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Bottom right coordinate in output resolution. Negative values alowed to go outside displayed area.
pub async fn set_image_processing_warp_fourcorners_bottomright(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.fourcorners.bottomright","params":{"property":"image.processing.warp.fourcorners.bottomright","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enable/Disable FourCorners warp
pub async fn set_image_processing_warp_fourcorners_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.fourcorners.enable","params":{"property":"image.processing.warp.fourcorners.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///[DEPRECATED] The height of the screen we are projecting on. Only used as in the ratio ScreenWidth/ScreenHeight, hence the unit is arbitrary.
pub async fn set_image_processing_warp_fourcorners_screenheight(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.fourcorners.screenheight","params":{"property":"image.processing.warp.fourcorners.screenheight","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///[DEPRECATED] The width of the screen we are projecting on. Only used as in the ratio ScreenWidth/ScreenHeight, hence the unit is arbitrary.
pub async fn set_image_processing_warp_fourcorners_screenwidth(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.fourcorners.screenwidth","params":{"property":"image.processing.warp.fourcorners.screenwidth","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Top left coordinate in output resolution. Negative values alowed to go outside displayed area.
pub async fn set_image_processing_warp_fourcorners_topleft(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.fourcorners.topleft","params":{"property":"image.processing.warp.fourcorners.topleft","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Top right coordinate in output resolution. Negative values alowed to go outside displayed area.
pub async fn set_image_processing_warp_fourcorners_topright(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.fourcorners.topright","params":{"property":"image.processing.warp.fourcorners.topright","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enable/Disable Position warp
pub async fn set_image_processing_warp_position_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.position.enable","params":{"property":"image.processing.warp.position.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The horizontal displacement factor relative to output resolution. From -HRes/2 to + HRes/2
pub async fn set_image_processing_warp_position_horizontal(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.position.horizontal","params":{"property":"image.processing.warp.position.horizontal","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The vertical displacement factor relative to output resolution. From -VRes/2 to +VRes/2
pub async fn set_image_processing_warp_position_vertical(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.position.vertical","params":{"property":"image.processing.warp.position.vertical","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The warp status.
pub async fn get_image_processing_warp_warpstatus(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.processing.warp.warpstatus\",\"params\":{\"property\":\"image.processing.warp.warpstatus\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Warp status description
pub async fn get_image_processing_warp_warpstatusdescription(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.processing.warp.warpstatusdescription\",\"params\":{\"property\":\"image.processing.warp.warpstatusdescription\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Enable/Disable Zoom warp
pub async fn set_image_processing_warp_zoom_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.zoom.enable","params":{"property":"image.processing.warp.zoom.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The zoom factor
pub async fn set_image_processing_warp_zoom_factor(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.processing.warp.zoom.factor","params":{"property":"image.processing.warp.zoom.factor","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Resulting zoomed resolution
pub async fn get_image_processing_warp_zoom_size(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.processing.warp.zoom.size\",\"params\":{\"property\":\"image.processing.warp.zoom.size\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The current resolution size (pixels x lines).
pub async fn get_image_resolution_alpha_size(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.resolution.alpha.size\",\"params\":{\"property\":\"image.resolution.alpha.size\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///List all resolutions that can currently be selected.
pub async fn get_image_resolution_availableresolutions(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.resolution.availableresolutions\",\"params\":{\"property\":\"image.resolution.availableresolutions\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The current resolution size (pixels x lines).
pub async fn get_image_resolution_beta_size(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.resolution.beta.size\",\"params\":{\"property\":\"image.resolution.beta.size\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The current resolution size (pixels x lines).
pub async fn get_image_resolution_display_size(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.resolution.display.size\",\"params\":{\"property\":\"image.resolution.display.size\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The current resolution size (pixels x lines).
pub async fn get_image_resolution_osd_size(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.resolution.osd.size\",\"params\":{\"property\":\"image.resolution.osd.size\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The current resolution size (pixels x lines).
pub async fn get_image_resolution_output_size(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.resolution.output.size\",\"params\":{\"property\":\"image.resolution.output.size\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The current resolution size (pixels x lines).
pub async fn get_image_resolution_processing_size(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.resolution.processing.size\",\"params\":{\"property\":\"image.resolution.processing.size\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The current resolution description.
pub async fn set_image_resolution_resolution(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.resolution.resolution","params":{"property":"image.resolution.resolution","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The current resolution size (pixels x lines).
pub async fn get_image_resolution_size(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.resolution.size\",\"params\":{\"property\":\"image.resolution.size\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Image color saturation. The value is normalized, 1 is default.
pub async fn set_image_saturation(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.saturation","params":{"property":"image.saturation","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Image sharpness. The value is normalized.
pub async fn set_image_sharpness(
    address: &str,
    value: u64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.sharpness","params":{"property":"image.sharpness","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Capabilities
pub async fn get_image_source_displayport_capabilities(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.source.displayport.capabilities\",\"params\":{\"property\":\"image.source.displayport.capabilities\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Source layout
pub async fn get_image_source_displayport_layout(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.source.displayport.layout\",\"params\":{\"property\":\"image.source.displayport.layout\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Name of the source
pub async fn get_image_source_displayport_name(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.source.displayport.name\",\"params\":{\"property\":\"image.source.displayport.name\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Capabilities
pub async fn get_image_source_hdmi_capabilities(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.source.hdmi.capabilities\",\"params\":{\"property\":\"image.source.hdmi.capabilities\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Source layout
pub async fn get_image_source_hdmi_layout(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.source.hdmi.layout\",\"params\":{\"property\":\"image.source.hdmi.layout\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Name of the source
pub async fn get_image_source_hdmi_name(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.source.hdmi.name\",\"params\":{\"property\":\"image.source.hdmi.name\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Darktime in us.
pub async fn set_image_stereo_darktime(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.stereo.darktime","params":{"property":"image.stereo.darktime","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Sync delay in us.
pub async fn set_image_stereo_glassync_delay(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.stereo.glassync.delay","params":{"property":"image.stereo.glassync.delay","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Maximum sync delay in us.
pub async fn get_image_stereo_glassync_delaymaximum(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.stereo.glassync.delaymaximum\",\"params\":{\"property\":\"image.stereo.glassync.delaymaximum\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Minimum sync delay in us.
pub async fn get_image_stereo_glassync_delayminimum(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.stereo.glassync.delayminimum\",\"params\":{\"property\":\"image.stereo.glassync.delayminimum\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Sync invert.
pub async fn set_image_stereo_glassync_invert(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.stereo.glassync.invert","params":{"property":"image.stereo.glassync.invert","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///swap which stereo frames belong to each other
pub async fn set_image_stereo_swapframepair(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.stereo.swapframepair","params":{"property":"image.stereo.swapframepair","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///When false, the service will deny outside property change requests
pub async fn get_image_testpattern_available(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.testpattern.available\",\"params\":{\"property\":\"image.testpattern.available\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Enables or disables the ability to upload download and delete custom testpatterns
pub async fn get_image_testpattern_enablefilemanagement(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.testpattern.enablefilemanagement\",\"params\":{\"property\":\"image.testpattern.enablefilemanagement\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The list of testpatterns to iterate in the viewing order.
pub async fn get_image_testpattern_iterator_calibration_list(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.testpattern.iterator.calibration.list\",\"params\":{\"property\":\"image.testpattern.iterator.calibration.list\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Return the size of the list of test patterns to iterate upon.
pub async fn get_image_testpattern_iterator_calibration_size(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.testpattern.iterator.calibration.size\",\"params\":{\"property\":\"image.testpattern.iterator.calibration.size\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The list of testpatterns to iterate in the viewing order.
pub async fn get_image_testpattern_iterator_iris_list(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.testpattern.iterator.iris.list\",\"params\":{\"property\":\"image.testpattern.iterator.iris.list\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Return the size of the list of test patterns to iterate upon.
pub async fn get_image_testpattern_iterator_iris_size(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.testpattern.iterator.iris.size\",\"params\":{\"property\":\"image.testpattern.iterator.iris.size\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The list of testpatterns to iterate in the viewing order.
pub async fn get_image_testpattern_iterator_optics_list(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.testpattern.iterator.optics.list\",\"params\":{\"property\":\"image.testpattern.iterator.optics.list\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Return the size of the list of test patterns to iterate upon.
pub async fn get_image_testpattern_iterator_optics_size(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.testpattern.iterator.optics.size\",\"params\":{\"property\":\"image.testpattern.iterator.optics.size\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The list of testpatterns to iterate in the viewing order.
pub async fn get_image_testpattern_iterator_shift_list(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.testpattern.iterator.shift.list\",\"params\":{\"property\":\"image.testpattern.iterator.shift.list\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Return the size of the list of test patterns to iterate upon.
pub async fn get_image_testpattern_iterator_shift_size(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.testpattern.iterator.shift.size\",\"params\":{\"property\":\"image.testpattern.iterator.shift.size\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The list of testpatterns to iterate in the viewing order.
pub async fn get_image_testpattern_iterator_stereo_list(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.testpattern.iterator.stereo.list\",\"params\":{\"property\":\"image.testpattern.iterator.stereo.list\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Return the size of the list of test patterns to iterate upon.
pub async fn get_image_testpattern_iterator_stereo_size(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.testpattern.iterator.stereo.size\",\"params\":{\"property\":\"image.testpattern.iterator.stereo.size\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The list of testpatterns to iterate in the viewing order.
pub async fn get_image_testpattern_iterator_xpr_list(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.testpattern.iterator.xpr.list\",\"params\":{\"property\":\"image.testpattern.iterator.xpr.list\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Return the size of the list of test patterns to iterate upon.
pub async fn get_image_testpattern_iterator_xpr_size(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.testpattern.iterator.xpr.size\",\"params\":{\"property\":\"image.testpattern.iterator.xpr.size\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The unique test pattern id to show as the NoSignal pattern
pub async fn set_image_testpattern_nosignal_pattern(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.testpattern.nosignal.pattern","params":{"property":"image.testpattern.nosignal.pattern","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The unique id of the selected pattern
pub async fn set_image_testpattern_selected(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.testpattern.selected","params":{"property":"image.testpattern.selected","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enable/disable the display of the selected pattern
pub async fn set_image_testpattern_show(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.testpattern.show","params":{"property":"image.testpattern.show","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Indicates whether or not the currently selected source has an active signal (true) or not (false)
pub async fn get_image_window_main_activesignal(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.window.main.activesignal\",\"params\":{\"property\":\"image.window.main.activesignal\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Window position
pub async fn get_image_window_main_position(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.window.main.position\",\"params\":{\"property\":\"image.window.main.position\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The scaling mode to apply to the source
pub async fn set_image_window_main_scalingmode(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.window.main.scalingmode","params":{"property":"image.window.main.scalingmode","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Window size
pub async fn get_image_window_main_size(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.window.main.size\",\"params\":{\"property\":\"image.window.main.size\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The source that is displayed in this window
pub async fn set_image_window_main_source(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"image.window.main.source","params":{"property":"image.window.main.source","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The reason(s) the source is not valid
pub async fn get_image_window_main_sourceinvalidityreasons(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"image.window.main.sourceinvalidityreasons\",\"params\":{\"property\":\"image.window.main.sourceinvalidityreasons\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///True if the certificate applies to this projector
pub async fn get_iot_certificate_applicable(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"iot.certificate.applicable\",\"params\":{\"property\":\"iot.certificate.applicable\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///True if a certificate was found
pub async fn get_iot_certificate_available(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"iot.certificate.available\",\"params\":{\"property\":\"iot.certificate.available\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///certificate Issuer info
pub async fn get_iot_certificate_issuer(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"iot.certificate.issuer\",\"params\":{\"property\":\"iot.certificate.issuer\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Result of the validity check of the certificate
pub async fn get_iot_certificate_valid(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"iot.certificate.valid\",\"params\":{\"property\":\"iot.certificate.valid\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///certificate Valid until this date
pub async fn get_iot_certificate_validityend(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"iot.certificate.validityend\",\"params\":{\"property\":\"iot.certificate.validityend\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///certificate Valid from this date
pub async fn get_iot_certificate_validitystart(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"iot.certificate.validitystart\",\"params\":{\"property\":\"iot.certificate.validitystart\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The current state of the connection.
pub async fn get_iot_cloudservices_connectionstate(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"iot.cloudservices.connectionstate\",\"params\":{\"property\":\"iot.cloudservices.connectionstate\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///True if IOT cloud connection is enabled, false otherwise
pub async fn set_iot_cloudservices_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"iot.cloudservices.enable","params":{"property":"iot.cloudservices.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///This is the address (hostname/ip) of the proxy.
pub async fn set_iot_proxy_address(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"iot.proxy.address","params":{"property":"iot.proxy.address","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///When enabled, iot connection will go over proxy specified in othe proxy parameters
pub async fn set_iot_proxy_enabled(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"iot.proxy.enabled","params":{"property":"iot.proxy.enabled","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Password used to log in to proxy.
pub async fn set_iot_proxy_password(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"iot.proxy.password","params":{"property":"iot.proxy.password","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The port to use for the proxy connection.
pub async fn set_iot_proxy_port(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"iot.proxy.port","params":{"property":"iot.proxy.port","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Username used to log in to proxy.
pub async fn set_iot_proxy_username(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"iot.proxy.username","params":{"property":"iot.proxy.username","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Indiates whether or not the user is required to accept the term agreement.
pub async fn get_iot_requiretermagreement(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"iot.requiretermagreement\",\"params\":{\"property\":\"iot.requiretermagreement\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Represents the users acknowledge of the IOT terms of agreement.
pub async fn set_iot_termagreement(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"iot.termagreement","params":{"property":"iot.termagreement","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Initial key repeat delay in milliseconds
pub async fn get_keydispatcher_repeatdelay(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"keydispatcher.repeatdelay\",\"params\":{\"property\":\"keydispatcher.repeatdelay\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Key repeat interval in milliseconds
pub async fn get_keydispatcher_repeatinterval(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"keydispatcher.repeatinterval\",\"params\":{\"property\":\"keydispatcher.repeatinterval\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Whether the device has carrier or not
pub async fn get_network_device_lan_carrier(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"network.device.lan.carrier\",\"params\":{\"property\":\"network.device.lan.carrier\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The configuration method of the device: auto or manual
pub async fn set_network_device_lan_configuration(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"network.device.lan.configuration","params":{"property":"network.device.lan.configuration","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The general type of the network device
pub async fn get_network_device_lan_devicetype(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"network.device.lan.devicetype\",\"params\":{\"property\":\"network.device.lan.devicetype\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The active hardware (MAC) address
pub async fn get_network_device_lan_hwaddress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"network.device.lan.hwaddress\",\"params\":{\"property\":\"network.device.lan.hwaddress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The current configuration for IP version 4
pub async fn get_network_device_lan_ip4config(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"network.device.lan.ip4config\",\"params\":{\"property\":\"network.device.lan.ip4config\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Get/set the manual configuration for IP version 4
pub async fn set_network_device_lan_ip4configmanual(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"network.device.lan.ip4configmanual","params":{"property":"network.device.lan.ip4configmanual","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The current configuration for IP version 6
pub async fn get_network_device_lan_ip6config(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"network.device.lan.ip6config\",\"params\":{\"property\":\"network.device.lan.ip6config\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Get/set the manual configuration for IP version 4
pub async fn set_network_device_lan_ip6configmanual(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"network.device.lan.ip6configmanual","params":{"property":"network.device.lan.ip6configmanual","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The speed of the device in Mbit/s
pub async fn get_network_device_lan_speed(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"network.device.lan.speed\",\"params\":{\"property\":\"network.device.lan.speed\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The current state of the device
pub async fn get_network_device_lan_state(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"network.device.lan.state\",\"params\":{\"property\":\"network.device.lan.state\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Additional information about the device state. Can be empty
pub async fn get_network_device_lan_stateinfo(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"network.device.lan.stateinfo\",\"params\":{\"property\":\"network.device.lan.stateinfo\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The domain name
pub async fn set_network_domain(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"network.domain","params":{"property":"network.domain","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The host name
pub async fn set_network_hostname(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"network.hostname","params":{"property":"network.hostname","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The Networking Service version
pub async fn get_network_version(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"network.version\",\"params\":{\"property\":\"network.version\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The number of notifications received and dismissed
pub async fn get_notification_count(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"notification.count\",\"params\":{\"property\":\"notification.count\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Current calibration state
pub async fn get_optics_focus_calibrationstate(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.focus.calibrationstate\",\"params\":{\"property\":\"optics.focus.calibrationstate\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Enabled state
pub async fn set_optics_focus_enabled(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"optics.focus.enabled","params":{"property":"optics.focus.enabled","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Forward limit reached
pub async fn get_optics_focus_limits_forward(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.focus.limits.forward\",\"params\":{\"property\":\"optics.focus.limits.forward\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Reverse limit reached
pub async fn get_optics_focus_limits_reverse(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.focus.limits.reverse\",\"params\":{\"property\":\"optics.focus.limits.reverse\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Maximum available position
pub async fn get_optics_focus_maxposition(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.focus.maxposition\",\"params\":{\"property\":\"optics.focus.maxposition\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Minimum available position
pub async fn get_optics_focus_minposition(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.focus.minposition\",\"params\":{\"property\":\"optics.focus.minposition\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Current position
pub async fn get_optics_focus_position(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.focus.position\",\"params\":{\"property\":\"optics.focus.position\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Safe to calibrate
pub async fn get_optics_focus_safetocalibrate(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.focus.safetocalibrate\",\"params\":{\"property\":\"optics.focus.safetocalibrate\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Safe to operate state
pub async fn get_optics_focus_safetooperate(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.focus.safetooperate\",\"params\":{\"property\":\"optics.focus.safetooperate\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Current state
pub async fn get_optics_focus_state(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.focus.state\",\"params\":{\"property\":\"optics.focus.state\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Desired target
pub async fn set_optics_focus_target(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"optics.focus.target","params":{"property":"optics.focus.target","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Property for selecting whether iris operation should be linked or individual
pub async fn set_optics_irisoperation(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"optics.irisoperation","params":{"property":"optics.irisoperation","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Description not provided
pub async fn get_optics_irisoperationavailable(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.irisoperationavailable\",\"params\":{\"property\":\"optics.irisoperationavailable\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Currently selected lens
pub async fn get_optics_lens_lens(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lens.lens\",\"params\":{\"property\":\"optics.lens.lens\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Current lens metadata
pub async fn get_optics_lens_metadata(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lens.metadata\",\"params\":{\"property\":\"optics.lens.metadata\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///List of available lens positions
pub async fn get_optics_lens_position_list(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lens.position.list\",\"params\":{\"property\":\"optics.lens.position.list\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Currently selected lens position
pub async fn set_optics_lens_position_selected(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"optics.lens.position.selected","params":{"property":"optics.lens.position.selected","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Lens positions stored focus position
pub async fn get_optics_lens_position_test1_focus(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lens.position.test1.focus\",\"params\":{\"property\":\"optics.lens.position.test1.focus\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Lens positions name
pub async fn get_optics_lens_position_test1_name(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lens.position.test1.name\",\"params\":{\"property\":\"optics.lens.position.test1.name\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Lens positions stored horizontal shift position
pub async fn get_optics_lens_position_test1_shifthorizontal(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lens.position.test1.shifthorizontal\",\"params\":{\"property\":\"optics.lens.position.test1.shifthorizontal\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Lens positions stored vertical shift position
pub async fn get_optics_lens_position_test1_shiftvertical(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lens.position.test1.shiftvertical\",\"params\":{\"property\":\"optics.lens.position.test1.shiftvertical\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Lens position status
pub async fn get_optics_lens_position_test1_status(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lens.position.test1.status\",\"params\":{\"property\":\"optics.lens.position.test1.status\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Lens positions stored zoom position
pub async fn get_optics_lens_position_test1_zoom(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lens.position.test1.zoom\",\"params\":{\"property\":\"optics.lens.position.test1.zoom\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Throw ratios of the selected lens
pub async fn get_optics_lens_throwratio(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lens.throwratio\",\"params\":{\"property\":\"optics.lens.throwratio\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Lens present
pub async fn get_optics_lenspresent(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lenspresent\",\"params\":{\"property\":\"optics.lenspresent\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Current calibration state
pub async fn get_optics_lensshift_horizontal_calibrationstate(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lensshift.horizontal.calibrationstate\",\"params\":{\"property\":\"optics.lensshift.horizontal.calibrationstate\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Enabled state
pub async fn set_optics_lensshift_horizontal_enabled(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"optics.lensshift.horizontal.enabled","params":{"property":"optics.lensshift.horizontal.enabled","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Forward limit reached
pub async fn get_optics_lensshift_horizontal_limits_forward(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lensshift.horizontal.limits.forward\",\"params\":{\"property\":\"optics.lensshift.horizontal.limits.forward\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Reverse limit reached
pub async fn get_optics_lensshift_horizontal_limits_reverse(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lensshift.horizontal.limits.reverse\",\"params\":{\"property\":\"optics.lensshift.horizontal.limits.reverse\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Maximum available position
pub async fn get_optics_lensshift_horizontal_maxposition(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lensshift.horizontal.maxposition\",\"params\":{\"property\":\"optics.lensshift.horizontal.maxposition\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Minimum available position
pub async fn get_optics_lensshift_horizontal_minposition(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lensshift.horizontal.minposition\",\"params\":{\"property\":\"optics.lensshift.horizontal.minposition\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Current position
pub async fn get_optics_lensshift_horizontal_position(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lensshift.horizontal.position\",\"params\":{\"property\":\"optics.lensshift.horizontal.position\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Safe to calibrate
pub async fn get_optics_lensshift_horizontal_safetocalibrate(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lensshift.horizontal.safetocalibrate\",\"params\":{\"property\":\"optics.lensshift.horizontal.safetocalibrate\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Safe to operate state
pub async fn get_optics_lensshift_horizontal_safetooperate(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lensshift.horizontal.safetooperate\",\"params\":{\"property\":\"optics.lensshift.horizontal.safetooperate\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Current state
pub async fn get_optics_lensshift_horizontal_state(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lensshift.horizontal.state\",\"params\":{\"property\":\"optics.lensshift.horizontal.state\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Desired target
pub async fn set_optics_lensshift_horizontal_target(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"optics.lensshift.horizontal.target","params":{"property":"optics.lensshift.horizontal.target","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Current calibration state
pub async fn get_optics_lensshift_vertical_calibrationstate(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lensshift.vertical.calibrationstate\",\"params\":{\"property\":\"optics.lensshift.vertical.calibrationstate\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Enabled state
pub async fn set_optics_lensshift_vertical_enabled(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"optics.lensshift.vertical.enabled","params":{"property":"optics.lensshift.vertical.enabled","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Forward limit reached
pub async fn get_optics_lensshift_vertical_limits_forward(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lensshift.vertical.limits.forward\",\"params\":{\"property\":\"optics.lensshift.vertical.limits.forward\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Reverse limit reached
pub async fn get_optics_lensshift_vertical_limits_reverse(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lensshift.vertical.limits.reverse\",\"params\":{\"property\":\"optics.lensshift.vertical.limits.reverse\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Maximum available position
pub async fn get_optics_lensshift_vertical_maxposition(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lensshift.vertical.maxposition\",\"params\":{\"property\":\"optics.lensshift.vertical.maxposition\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Minimum available position
pub async fn get_optics_lensshift_vertical_minposition(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lensshift.vertical.minposition\",\"params\":{\"property\":\"optics.lensshift.vertical.minposition\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Current position
pub async fn get_optics_lensshift_vertical_position(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lensshift.vertical.position\",\"params\":{\"property\":\"optics.lensshift.vertical.position\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Safe to calibrate
pub async fn get_optics_lensshift_vertical_safetocalibrate(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lensshift.vertical.safetocalibrate\",\"params\":{\"property\":\"optics.lensshift.vertical.safetocalibrate\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Safe to operate state
pub async fn get_optics_lensshift_vertical_safetooperate(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lensshift.vertical.safetooperate\",\"params\":{\"property\":\"optics.lensshift.vertical.safetooperate\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Current state
pub async fn get_optics_lensshift_vertical_state(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.lensshift.vertical.state\",\"params\":{\"property\":\"optics.lensshift.vertical.state\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Desired target
pub async fn set_optics_lensshift_vertical_target(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"optics.lensshift.vertical.target","params":{"property":"optics.lensshift.vertical.target","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Position of shutter
pub async fn get_optics_shutter_position(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.shutter.position\",\"params\":{\"property\":\"optics.shutter.position\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Target position of shutter
pub async fn set_optics_shutter_target(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"optics.shutter.target","params":{"property":"optics.shutter.target","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Current calibration state
pub async fn get_optics_zoom_calibrationstate(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.zoom.calibrationstate\",\"params\":{\"property\":\"optics.zoom.calibrationstate\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Enabled state
pub async fn set_optics_zoom_enabled(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"optics.zoom.enabled","params":{"property":"optics.zoom.enabled","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Forward limit reached
pub async fn get_optics_zoom_limits_forward(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.zoom.limits.forward\",\"params\":{\"property\":\"optics.zoom.limits.forward\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Reverse limit reached
pub async fn get_optics_zoom_limits_reverse(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.zoom.limits.reverse\",\"params\":{\"property\":\"optics.zoom.limits.reverse\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Maximum available position
pub async fn get_optics_zoom_maxposition(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.zoom.maxposition\",\"params\":{\"property\":\"optics.zoom.maxposition\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Minimum available position
pub async fn get_optics_zoom_minposition(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.zoom.minposition\",\"params\":{\"property\":\"optics.zoom.minposition\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Current position
pub async fn get_optics_zoom_position(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.zoom.position\",\"params\":{\"property\":\"optics.zoom.position\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Safe to calibrate
pub async fn get_optics_zoom_safetocalibrate(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.zoom.safetocalibrate\",\"params\":{\"property\":\"optics.zoom.safetocalibrate\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Safe to operate state
pub async fn get_optics_zoom_safetooperate(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.zoom.safetooperate\",\"params\":{\"property\":\"optics.zoom.safetooperate\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Current state
pub async fn get_optics_zoom_state(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"optics.zoom.state\",\"params\":{\"property\":\"optics.zoom.state\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Desired target
pub async fn set_optics_zoom_target(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"optics.zoom.target","params":{"property":"optics.zoom.target","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The list of available domains for profiles.
pub async fn get_profile_domains(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"profile.domains\",\"params\":{\"property\":\"profile.domains\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///All preset assigments.
pub async fn get_profile_presetassignments(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"profile.presetassignments\",\"params\":{\"property\":\"profile.presetassignments\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The list of created profiles.
pub async fn get_profile_profiles(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"profile.profiles\",\"params\":{\"property\":\"profile.profiles\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn set_property_notifyself(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"property.notifyself","params":{"property":"property.notifyself","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enable/disable PJLink authentication procedure
pub async fn get_protocols_pjlink_authenticationrequired(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"protocols.pjlink.authenticationrequired\",\"params\":{\"property\":\"protocols.pjlink.authenticationrequired\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Enable/disable PJLink
pub async fn get_protocols_pjlink_enable(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"protocols.pjlink.enable\",\"params\":{\"property\":\"protocols.pjlink.enable\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The address of the remote control that the projector will respond to
pub async fn set_remotecontrol_address(
    address: &str,
    value: u64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"remotecontrol.address","params":{"property":"remotecontrol.address","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The broadcast address
pub async fn set_remotecontrol_broadcastaddress(
    address: &str,
    value: u64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"remotecontrol.broadcastaddress","params":{"property":"remotecontrol.broadcastaddress","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enable or disable the IR sensor
pub async fn set_remotecontrol_sensors_front_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"remotecontrol.sensors.front.enable","params":{"property":"remotecontrol.sensors.front.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The display name of the IR sensor
pub async fn get_remotecontrol_sensors_front_name(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"remotecontrol.sensors.front.name\",\"params\":{\"property\":\"remotecontrol.sensors.front.name\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Enable or disable the IR sensor
pub async fn set_remotecontrol_sensors_rear_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"remotecontrol.sensors.rear.enable","params":{"property":"remotecontrol.sensors.rear.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The display name of the IR sensor
pub async fn get_remotecontrol_sensors_rear_name(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"remotecontrol.sensors.rear.name\",\"params\":{\"property\":\"remotecontrol.sensors.rear.name\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Enable or disable the IR sensor
pub async fn set_remotecontrol_sensors_side_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"remotecontrol.sensors.side.enable","params":{"property":"remotecontrol.sensors.side.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The display name of the IR sensor
pub async fn get_remotecontrol_sensors_side_name(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"remotecontrol.sensors.side.name\",\"params\":{\"property\":\"remotecontrol.sensors.side.name\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Enable or disable the IR sensor
pub async fn set_remotecontrol_sensors_wiredremote_layer1_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"remotecontrol.sensors.wiredremote-layer1.enable","params":{"property":"remotecontrol.sensors.wiredremote-layer1.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The display name of the IR sensor
pub async fn get_remotecontrol_sensors_wiredremote_layer1_name(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"remotecontrol.sensors.wiredremote-layer1.name\",\"params\":{\"property\":\"remotecontrol.sensors.wiredremote-layer1.name\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Enable or disable the IR sensor
pub async fn set_remotecontrol_sensors_wiredremote_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"remotecontrol.sensors.wiredremote.enable","params":{"property":"remotecontrol.sensors.wiredremote.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The display name of the IR sensor
pub async fn get_remotecontrol_sensors_wiredremote_name(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"remotecontrol.sensors.wiredremote.name\",\"params\":{\"property\":\"remotecontrol.sensors.wiredremote.name\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///List of all scheduled actions' name.
pub async fn get_scheduler_actions(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"scheduler.actions\",\"params\":{\"property\":\"scheduler.actions\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///List of all available action types.
pub async fn get_scheduler_actiontypes(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"scheduler.actiontypes\",\"params\":{\"property\":\"scheduler.actiontypes\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The next task(s) that will be executed. (Can be more than one task if multiple tasks have the exact same execution time.)
pub async fn get_scheduler_nextscheduledaction(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"scheduler.nextscheduledaction\",\"params\":{\"property\":\"scheduler.nextscheduledaction\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Return True if action is paused, otherwise return False.
pub async fn set_scheduler_pause(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"scheduler.pause","params":{"property":"scheduler.pause","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///List of all scheduled actions data.
pub async fn set_scheduler_scheduledactions(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"scheduler.scheduledactions","params":{"property":"scheduler.scheduledactions","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///List of all available transitions for system action.
pub async fn get_scheduler_transitions(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"scheduler.transitions\",\"params\":{\"property\":\"scheduler.transitions\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///[DEPRECATED] Replaced by image.processing.colormapping.hdrboost
pub async fn set_screen_hdrboost(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"screen.hdrboost","params":{"property":"screen.hdrboost","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The maximum luminance measured on the screen in cd/m2
pub async fn set_screen_luminance(
    address: &str,
    value: f64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"screen.luminance","params":{"property":"screen.luminance","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The calculated aspect of the ScreenWidth/ScreenHeight
pub async fn get_screen_screenaspect(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"screen.screenaspect\",\"params\":{\"property\":\"screen.screenaspect\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The width and height of the screen we are projecting on.
pub async fn set_screen_screensize(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"screen.screensize","params":{"property":"screen.screensize","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Counter value
pub async fn get_statistics_laser_plate01_bank01_runtimeseconds_value(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"statistics.laser.plate01.bank01.runtimeseconds.value\",\"params\":{\"property\":\"statistics.laser.plate01.bank01.runtimeseconds.value\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Counter value
pub async fn get_statistics_laser_plate01_bank02_runtimeseconds_value(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"statistics.laser.plate01.bank02.runtimeseconds.value\",\"params\":{\"property\":\"statistics.laser.plate01.bank02.runtimeseconds.value\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Counter value
pub async fn get_statistics_laser_plate02_bank01_runtimeseconds_value(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"statistics.laser.plate02.bank01.runtimeseconds.value\",\"params\":{\"property\":\"statistics.laser.plate02.bank01.runtimeseconds.value\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Counter value
pub async fn get_statistics_laser_plate02_bank02_runtimeseconds_value(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"statistics.laser.plate02.bank02.runtimeseconds.value\",\"params\":{\"property\":\"statistics.laser.plate02.bank02.runtimeseconds.value\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Counter value
pub async fn get_statistics_maintenance_actuator_value(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"statistics.maintenance.actuator.value\",\"params\":{\"property\":\"statistics.maintenance.actuator.value\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Counter value
pub async fn get_statistics_maintenance_colorwheel_value(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"statistics.maintenance.colorwheel.value\",\"params\":{\"property\":\"statistics.maintenance.colorwheel.value\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Counter value
pub async fn get_statistics_maintenance_phosphorwheel_value(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"statistics.maintenance.phosphorwheel.value\",\"params\":{\"property\":\"statistics.maintenance.phosphorwheel.value\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Counter value
pub async fn get_statistics_projector_value(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"statistics.projector.value\",\"params\":{\"property\":\"statistics.projector.value\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Article number.
pub async fn get_system_articlenumber(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.articlenumber\",\"params\":{\"property\":\"system.articlenumber\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Article number of installed color wheel
pub async fn get_system_colorwheel(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.colorwheel\",\"params\":{\"property\":\"system.colorwheel\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Name of installed color wheel
pub async fn get_system_colorwheelname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.colorwheelname\",\"params\":{\"property\":\"system.colorwheelname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///An array of all available time zones and their information.
pub async fn get_system_date_availabletimezones(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.date.availabletimezones\",\"params\":{\"property\":\"system.date.availabletimezones\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///True if there is a connection to the NTP-server.
pub async fn get_system_date_ntp_connected(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.date.ntp.connected\",\"params\":{\"property\":\"system.date.ntp.connected\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///True if NTP time synchronization should be used.
pub async fn get_system_date_ntp_enabled(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.date.ntp.enabled\",\"params\":{\"property\":\"system.date.ntp.enabled\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The NTP server hostname or address.
pub async fn get_system_date_ntp_server(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.date.ntp.server\",\"params\":{\"property\":\"system.date.ntp.server\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The configured time zone of the system.
pub async fn get_system_date_timezone(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.date.timezone\",\"params\":{\"property\":\"system.date.timezone\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Returns true if state is available for this projector
pub async fn get_system_eco_available(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.eco.available\",\"params\":{\"property\":\"system.eco.available\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Enable/disable the use of this state. Check if available first.
pub async fn set_system_eco_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"system.eco.enable","params":{"property":"system.eco.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Family name.
pub async fn get_system_familyname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.familyname\",\"params\":{\"property\":\"system.familyname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Firmware version.
pub async fn get_system_firmwareversion(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.firmwareversion\",\"params\":{\"property\":\"system.firmwareversion\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The current global health state of the projector. State error means the projector can not safely be operated. Warning means the show can go on, but it is strongly advised to find the cause of the warning and solve it. Normal means the projector is performing normally.
pub async fn get_system_health(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.health\",\"params\":{\"property\":\"system.health\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///State to transition to when the unit is started
pub async fn set_system_initialstate(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"system.initialstate","params":{"property":"system.initialstate","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Applicability of the license file.
pub async fn get_system_license_applicable(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.license.applicable\",\"params\":{\"property\":\"system.license.applicable\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Availability of a license file.
pub async fn get_system_license_available(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.license.available\",\"params\":{\"property\":\"system.license.available\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///A dictionary of options and their values.
pub async fn get_system_license_options(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.license.options\",\"params\":{\"property\":\"system.license.options\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Shows if registering the product is mandatory or optional
pub async fn get_system_license_register_mandatory(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.license.register.mandatory\",\"params\":{\"property\":\"system.license.register.mandatory\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///True when the product has been successfully registered.
pub async fn get_system_license_register_valid(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.license.register.valid\",\"params\":{\"property\":\"system.license.register.valid\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Validity of the license file.
pub async fn get_system_license_valid(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.license.valid\",\"params\":{\"property\":\"system.license.valid\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Model name.
pub async fn get_system_modelname(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.modelname\",\"params\":{\"property\":\"system.modelname\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Custom name for this device.
pub async fn get_system_name(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.name\",\"params\":{\"property\":\"system.name\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Time (in seconds) to wait in this state before entering lower state.
pub async fn set_system_on_timeout_duration(
    address: &str,
    value: u64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"system.on.timeout.duration","params":{"property":"system.on.timeout.duration","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enable/disable the timeout.
pub async fn set_system_on_timeout_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"system.on.timeout.enable","params":{"property":"system.on.timeout.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The remaining amount of seconds before the timer will timeout.
pub async fn get_system_on_timeout_remaining(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.on.timeout.remaining\",\"params\":{\"property\":\"system.on.timeout.remaining\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///List of available operational modes
pub async fn get_system_operationalmodes_availablemodes(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.operationalmodes.availablemodes\",\"params\":{\"property\":\"system.operationalmodes.availablemodes\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The currently active operational mode
pub async fn set_system_operationalmodes_mode(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"system.operationalmodes.mode","params":{"property":"system.operationalmodes.mode","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Time (in seconds) to wait in this state before entering lower state.
pub async fn set_system_ready_timeout_duration(
    address: &str,
    value: u64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"system.ready.timeout.duration","params":{"property":"system.ready.timeout.duration","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enable/disable the timeout.
pub async fn set_system_ready_timeout_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"system.ready.timeout.enable","params":{"property":"system.ready.timeout.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The remaining amount of seconds before the timer will timeout.
pub async fn get_system_ready_timeout_remaining(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.ready.timeout.remaining\",\"params\":{\"property\":\"system.ready.timeout.remaining\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Reset progress: [0..100]
pub async fn get_system_resetprogress(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.resetprogress\",\"params\":{\"property\":\"system.resetprogress\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_system_resetresult(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.resetresult\",\"params\":{\"property\":\"system.resetresult\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Reset status
pub async fn get_system_resetstatus(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.resetstatus\",\"params\":{\"property\":\"system.resetstatus\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Serial number.
pub async fn get_system_serialnumber(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.serialnumber\",\"params\":{\"property\":\"system.serialnumber\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Returns true if state is available for this projector
pub async fn get_system_standby_available(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.standby.available\",\"params\":{\"property\":\"system.standby.available\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Enable/disable the use of this state. Check if available first.
pub async fn set_system_standby_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"system.standby.enable","params":{"property":"system.standby.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Time (in seconds) to wait in this state before entering lower state.
pub async fn set_system_standby_timeout_duration(
    address: &str,
    value: u64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"system.standby.timeout.duration","params":{"property":"system.standby.timeout.duration","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enable/disable the timeout.
pub async fn set_system_standby_timeout_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"system.standby.timeout.enable","params":{"property":"system.standby.timeout.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The remaining amount of seconds before the timer will timeout.
pub async fn get_system_standby_timeout_remaining(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.standby.timeout.remaining\",\"params\":{\"property\":\"system.standby.timeout.remaining\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The current state of the unit
pub async fn get_system_state(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.state\",\"params\":{\"property\":\"system.state\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The state the unit is transitioning into
pub async fn get_system_targetstate(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"system.targetstate\",\"params\":{\"property\":\"system.targetstate\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///True and available when the user has end user access privileges.
pub async fn get_ui_access_enduser(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"ui.access.enduser\",\"params\":{\"property\":\"ui.access.enduser\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn set_ui_backlight_state(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.backlight.state","params":{"property":"ui.backlight.state","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The amount of seconds after which the lcd backlight will be switched off when the menu and stealth mode are not active and there is no activity.
pub async fn set_ui_backlight_timeout(
    address: &str,
    value: u64) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.backlight.timeout","params":{"property":"ui.backlight.timeout","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Description not provided
pub async fn get_ui_hasstealthmode(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"ui.hasstealthmode\",\"params\":{\"property\":\"ui.hasstealthmode\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The user interface language
pub async fn set_ui_language(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.language","params":{"property":"ui.language","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Drawing commands in the form of a JSON object
pub async fn set_ui_layer_advancedblend_drawing(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.advancedblend.drawing","params":{"property":"ui.layer.advancedblend.drawing","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enable or disable the layer
pub async fn set_ui_layer_advancedblend_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.advancedblend.enable","params":{"property":"ui.layer.advancedblend.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Color palette that can be used when drawing the blend layer
pub async fn set_ui_layer_advancedblend_palette(
    address: &str,
    value: std::vec::Vec<String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.advancedblend.palette","params":{"property":"ui.layer.advancedblend.palette","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The edge color, e.g '#ffff00' or 'rgba(255,255,0,0.5)
pub async fn set_ui_layer_basicblacklevel_color(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.basicblacklevel.color","params":{"property":"ui.layer.basicblacklevel.color","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enable or disable the layer
pub async fn set_ui_layer_basicblacklevel_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.basicblacklevel.enable","params":{"property":"ui.layer.basicblacklevel.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Toggle edge selection
pub async fn set_ui_layer_basicblacklevel_selection(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.basicblacklevel.selection","params":{"property":"ui.layer.basicblacklevel.selection","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The color to use for selected edges, e.g '#ff0000' or 'rgba(255,0,0,0.5)'
pub async fn set_ui_layer_basicblacklevel_selectioncolor(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.basicblacklevel.selectioncolor","params":{"property":"ui.layer.basicblacklevel.selectioncolor","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The edge color, e.g '#ffff00' or 'rgba(255,255,0,0.5)
pub async fn set_ui_layer_basicblend_color(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.basicblend.color","params":{"property":"ui.layer.basicblend.color","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enable or disable the layer
pub async fn set_ui_layer_basicblend_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.basicblend.enable","params":{"property":"ui.layer.basicblend.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Toggle edge selection
pub async fn set_ui_layer_basicblend_selection(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.basicblend.selection","params":{"property":"ui.layer.basicblend.selection","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The color to use for selected edges, e.g '#ff0000' or 'rgba(255,0,0,0.5)'
pub async fn set_ui_layer_basicblend_selectioncolor(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.basicblend.selectioncolor","params":{"property":"ui.layer.basicblend.selectioncolor","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///(Optional) Show/hide a border/outline of the screen
pub async fn set_ui_layer_blank_border(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.blank.border","params":{"property":"ui.layer.blank.border","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///(Optional) Specify the border color, e.g '#ff0000' or 'rgba(255,0,0,0.75)'
pub async fn set_ui_layer_blank_bordercolor(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.blank.bordercolor","params":{"property":"ui.layer.blank.bordercolor","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enable or disable the layer. When enabled, the screen will be covered in black.
pub async fn set_ui_layer_blank_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.blank.enable","params":{"property":"ui.layer.blank.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///A single UTF character from the Barco-Icons font set.
pub async fn set_ui_layer_blank_icon(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.blank.icon","params":{"property":"ui.layer.blank.icon","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Used only for persisting whether or not to show the layer
pub async fn set_ui_layer_blank_show(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.blank.show","params":{"property":"ui.layer.blank.show","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Show or hide the text
pub async fn set_ui_layer_blank_showtext(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.blank.showtext","params":{"property":"ui.layer.blank.showtext","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///(Optional) Specify a text to show at the center of the screen
pub async fn set_ui_layer_blank_text(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.blank.text","params":{"property":"ui.layer.blank.text","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The color to use for unselected corners, e.g '#ff0000' or 'rgba(255,0,0,0.75)'
pub async fn set_ui_layer_fourcorner_cornercolor(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.fourcorner.cornercolor","params":{"property":"ui.layer.fourcorner.cornercolor","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enable or disable the layer
pub async fn set_ui_layer_fourcorner_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.fourcorner.enable","params":{"property":"ui.layer.fourcorner.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The line color, e.g '#ffff00' or 'rgba(255,255,0,0.5)'
pub async fn set_ui_layer_fourcorner_linecolor(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.fourcorner.linecolor","params":{"property":"ui.layer.fourcorner.linecolor","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Show or hide lines between the corners
pub async fn set_ui_layer_fourcorner_lines(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.fourcorner.lines","params":{"property":"ui.layer.fourcorner.lines","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Toggle corner selection
pub async fn set_ui_layer_fourcorner_selection(
    address: &str,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.fourcorner.selection","params":{"property":"ui.layer.fourcorner.selection","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The color to use for selected corners, e.g '#ff0000' or 'rgba(255,0,0,0.75)'
pub async fn set_ui_layer_fourcorner_selectioncolor(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.fourcorner.selectioncolor","params":{"property":"ui.layer.fourcorner.selectioncolor","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Default color for grid points, e.g '#ff0000' or 'rgba(0,0,255,0.5)'
pub async fn set_ui_layer_grid_color(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.grid.color","params":{"property":"ui.layer.grid.color","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enable or disable the layer
pub async fn set_ui_layer_grid_enable(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.grid.enable","params":{"property":"ui.layer.grid.enable","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///[DEPRECATED] Use ShowLines instead. This is for backwards compability
pub async fn set_ui_layer_grid_lines(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.grid.lines","params":{"property":"ui.layer.grid.lines","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///List of row,column and color triplets for marking points in the grid. The color is specified as '#ff00ff' or 'rgba(0,255,255,0.75)'
pub async fn set_ui_layer_grid_mark(
    address: &str,
    value: std::vec::Vec<std::collections::HashMap<String, String>>) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.grid.mark","params":{"property":"ui.layer.grid.mark","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Number of grid points
pub async fn set_ui_layer_grid_points(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.grid.points","params":{"property":"ui.layer.grid.points","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Toggle drawing lines between grid points
pub async fn set_ui_layer_grid_showlines(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.grid.showlines","params":{"property":"ui.layer.grid.showlines","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Toggle drawing grid points
pub async fn set_ui_layer_grid_showpoints(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.layer.grid.showpoints","params":{"property":"ui.layer.grid.showpoints","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Preferred unit for display of lengths and distances
pub async fn set_ui_length(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.length","params":{"property":"ui.length","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Show or hide the lens menu
pub async fn set_ui_lensmenu(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.lensmenu","params":{"property":"ui.lensmenu","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Preferred unit for luminance
pub async fn set_ui_luminance(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.luminance","params":{"property":"ui.luminance","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Show or hide the menu
pub async fn set_ui_menu(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.menu","params":{"property":"ui.menu","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Placement of menu related to full screen.
pub async fn set_ui_menuposition(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.menuposition","params":{"property":"ui.menuposition","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The persistent message to show on the screen. Rich text allowed
pub async fn get_ui_message(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"ui.message\",\"params\":{\"property\":\"ui.message\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Placement of the message relative to full screen
pub async fn get_ui_messageposition(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"ui.messageposition\",\"params\":{\"property\":\"ui.messageposition\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Minimize the menu when it is enabled
pub async fn set_ui_minimize(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.minimize","params":{"property":"ui.minimize","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enable or disable on screen display
pub async fn set_ui_osd(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.osd","params":{"property":"ui.osd","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Show or hide the pattern menu shortcut
pub async fn set_ui_patternmenu(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.patternmenu","params":{"property":"ui.patternmenu","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///When true, a dialog shows info about powering down
pub async fn set_ui_poweroffhint(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.poweroffhint","params":{"property":"ui.poweroffhint","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The unit.
pub async fn get_ui_sensors_airpressure_unit(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"ui.sensors.airpressure.unit\",\"params\":{\"property\":\"ui.sensors.airpressure.unit\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The value.
pub async fn get_ui_sensors_airpressure_value(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"ui.sensors.airpressure.value\",\"params\":{\"property\":\"ui.sensors.airpressure.value\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The unit.
pub async fn get_ui_sensors_ambienttemperature_unit(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"ui.sensors.ambienttemperature.unit\",\"params\":{\"property\":\"ui.sensors.ambienttemperature.unit\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The value.
pub async fn get_ui_sensors_ambienttemperature_value(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"ui.sensors.ambienttemperature.value\",\"params\":{\"property\":\"ui.sensors.ambienttemperature.value\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The unit.
pub async fn get_ui_sensors_humidity_unit(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"ui.sensors.humidity.unit\",\"params\":{\"property\":\"ui.sensors.humidity.unit\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The value.
pub async fn get_ui_sensors_humidity_value(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"ui.sensors.humidity.value\",\"params\":{\"property\":\"ui.sensors.humidity.value\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The unit.
pub async fn get_ui_sensors_mainsvoltage_unit(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"ui.sensors.mainsvoltage.unit\",\"params\":{\"property\":\"ui.sensors.mainsvoltage.unit\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The value.
pub async fn get_ui_sensors_mainsvoltage_value(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"ui.sensors.mainsvoltage.value\",\"params\":{\"property\":\"ui.sensors.mainsvoltage.value\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Show a persistent message on the screen
pub async fn get_ui_showmessage(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"ui.showmessage\",\"params\":{\"property\":\"ui.showmessage\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Show or hide the input source shortcut menu
pub async fn set_ui_sourcemenu(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.sourcemenu","params":{"property":"ui.sourcemenu","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Show/hide the source signal information popup
pub async fn set_ui_sourcesignal(
    address: &str,
    value: bool) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.sourcesignal","params":{"property":"ui.sourcesignal","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Placement of the source signal information
pub async fn set_ui_sourcesignalposition(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.sourcesignalposition","params":{"property":"ui.sourcesignalposition","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Value to indicate if the splash screen should be shown
pub async fn get_ui_splashscreen_show(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"ui.splashscreen.show\",\"params\":{\"property\":\"ui.splashscreen.show\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Time (in seconds) the splash screen is shown on startup
pub async fn get_ui_splashscreen_timeoutseconds(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"ui.splashscreen.timeoutseconds\",\"params\":{\"property\":\"ui.splashscreen.timeoutseconds\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///When the projector is in stealth mode, all controllable LEDs are switched off.
pub async fn set_ui_stealthmode(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.stealthmode","params":{"property":"ui.stealthmode","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Preferred unit for display of temperature values
pub async fn set_ui_temperature(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.temperature","params":{"property":"ui.temperature","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///The theme setting of the user interface.
pub async fn set_ui_theme(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.theme","params":{"property":"ui.theme","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Enables or disables the touchscreen test.
pub async fn get_ui_touchscreen_mode(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"ui.touchscreen.mode\",\"params\":{\"property\":\"ui.touchscreen.mode\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Current status of the touchscreen text
pub async fn set_ui_touchscreen_status(
    address: &str,
    value: String) -> APICallResult {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.set","id":"ui.touchscreen.status","params":{"property":"ui.touchscreen.status","value":value}}).to_string();
    let res = client.post(address).body(payload).send().await?;
    let res_body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&res_body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_admin_activesessioncount(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.admin.activesessioncount\",\"params\":{\"property\":\"user.admin.activesessioncount\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_admin_enabled(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.admin.enabled\",\"params\":{\"property\":\"user.admin.enabled\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_admin_group(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.admin.group\",\"params\":{\"property\":\"user.admin.group\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_admin_publickey(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.admin.publickey\",\"params\":{\"property\":\"user.admin.publickey\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_admin_username(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.admin.username\",\"params\":{\"property\":\"user.admin.username\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_admin_userslug(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.admin.userslug\",\"params\":{\"property\":\"user.admin.userslug\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Require authentication of all users
pub async fn get_user_authenticationrequired(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.authenticationrequired\",\"params\":{\"property\":\"user.authenticationrequired\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The available user groups
pub async fn get_user_availablegroups(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.availablegroups\",\"params\":{\"property\":\"user.availablegroups\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_currentuser_activesessioncount(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.currentuser.activesessioncount\",\"params\":{\"property\":\"user.currentuser.activesessioncount\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_currentuser_enabled(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.currentuser.enabled\",\"params\":{\"property\":\"user.currentuser.enabled\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_currentuser_group(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.currentuser.group\",\"params\":{\"property\":\"user.currentuser.group\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_currentuser_publickey(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.currentuser.publickey\",\"params\":{\"property\":\"user.currentuser.publickey\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_currentuser_username(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.currentuser.username\",\"params\":{\"property\":\"user.currentuser.username\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_currentuser_userslug(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.currentuser.userslug\",\"params\":{\"property\":\"user.currentuser.userslug\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///The list of all users
pub async fn get_user_list(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.list\",\"params\":{\"property\":\"user.list\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Allow authenticating users with pin code
pub async fn get_user_pincodeauthenticationenabled(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.pincodeauthenticationenabled\",\"params\":{\"property\":\"user.pincodeauthenticationenabled\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_poweruser_activesessioncount(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.poweruser.activesessioncount\",\"params\":{\"property\":\"user.poweruser.activesessioncount\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_poweruser_enabled(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.poweruser.enabled\",\"params\":{\"property\":\"user.poweruser.enabled\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_poweruser_group(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.poweruser.group\",\"params\":{\"property\":\"user.poweruser.group\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_poweruser_publickey(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.poweruser.publickey\",\"params\":{\"property\":\"user.poweruser.publickey\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_poweruser_username(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.poweruser.username\",\"params\":{\"property\":\"user.poweruser.username\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_poweruser_userslug(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.poweruser.userslug\",\"params\":{\"property\":\"user.poweruser.userslug\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Challenge for resetting the administrator user
pub async fn get_user_resetadministratorchallenge(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.resetadministratorchallenge\",\"params\":{\"property\":\"user.resetadministratorchallenge\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_testuser_activesessioncount(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.testuser.activesessioncount\",\"params\":{\"property\":\"user.testuser.activesessioncount\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_testuser_enabled(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.testuser.enabled\",\"params\":{\"property\":\"user.testuser.enabled\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_testuser_group(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.testuser.group\",\"params\":{\"property\":\"user.testuser.group\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_testuser_publickey(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.testuser.publickey\",\"params\":{\"property\":\"user.testuser.publickey\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_testuser_username(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.testuser.username\",\"params\":{\"property\":\"user.testuser.username\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_testuser_userslug(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.testuser.userslug\",\"params\":{\"property\":\"user.testuser.userslug\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_user_activesessioncount(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.user.activesessioncount\",\"params\":{\"property\":\"user.user.activesessioncount\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_user_enabled(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.user.enabled\",\"params\":{\"property\":\"user.user.enabled\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_user_group(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.user.group\",\"params\":{\"property\":\"user.user.group\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_user_publickey(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.user.publickey\",\"params\":{\"property\":\"user.user.publickey\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_user_username(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.user.username\",\"params\":{\"property\":\"user.user.username\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
///Description not provided
pub async fn get_user_user_userslug(address: &str) -> APICallResult {
    let client = reqwest::Client::new();
    let res = client.post(address).body("{\"jsonrpc\":\"2.0\",\"method\":\"property.get\",\"id\":\"user.user.userslug\",\"params\":{\"property\":\"user.user.userslug\"}}").send().await?;
    let body = res.text().await?;
    let res: APICallResponse = serde_json::from_str(&body)?;
    Ok(res)
}
