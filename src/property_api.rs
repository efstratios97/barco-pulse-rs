
// Copyright (c) 2026 Efstratios Pahis
// SPDX-License-Identifier: MPL-2.0

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
///The error message in case of a backup error state.
pub async fn get_backup_auto_error(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"backup.auto.error","params":{"property":"backup.auto.error"}});

    session.call(payload).await
}
///Initial IP address for backup actions.
pub async fn get_backup_auto_ipaddress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"backup.auto.ipaddress","params":{"property":"backup.auto.ipaddress"}});

    session.call(payload).await
}
///Date on which SyncState became Asynchronous
pub async fn get_backup_auto_outofsyncdate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"backup.auto.outofsyncdate","params":{"property":"backup.auto.outofsyncdate"}});

    session.call(payload).await
}
///The current state of the automatic backup
pub async fn get_backup_auto_state(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"backup.auto.state","params":{"property":"backup.auto.state"}});

    session.call(payload).await
}
///Whether the backup is in sync
pub async fn get_backup_auto_syncstate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"backup.auto.syncstate","params":{"property":"backup.auto.syncstate"}});

    session.call(payload).await
}
///Progress for the selftest.
pub async fn get_diagnostics_selftest_progress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"diagnostics.selftest.progress","params":{"property":"diagnostics.selftest.progress"}});

    session.call(payload).await
}
///The result and overview of all executed selftests
pub async fn get_diagnostics_selftest_result(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"diagnostics.selftest.result","params":{"property":"diagnostics.selftest.result"}});

    session.call(payload).await
}
///The current status of the executed test-list
pub async fn get_diagnostics_selftest_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"diagnostics.selftest.status","params":{"property":"diagnostics.selftest.status"}});

    session.call(payload).await
}
///Artnet enabled or not
pub async fn set_dmx_artnet(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "dmx.artnet",
        "params": {
            "property": "dmx.artnet",
            "value": value
        }
    });

    session.call(payload).await
}
///Artnet net selection
pub async fn set_dmx_artnetnet(
    session: &mut PulseSession,
    value: u64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "dmx.artnetnet",
        "params": {
            "property": "dmx.artnetnet",
            "value": value
        }
    });

    session.call(payload).await
}
///Artnet universe selection
pub async fn set_dmx_artnetuniverse(
    session: &mut PulseSession,
    value: u64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "dmx.artnetuniverse",
        "params": {
            "property": "dmx.artnetuniverse",
            "value": value
        }
    });

    session.call(payload).await
}
///Current mode
pub async fn set_dmx_mode(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "dmx.mode",
        "params": {
            "property": "dmx.mode",
            "value": value
        }
    });

    session.call(payload).await
}
///Description for the dmx channel
pub async fn get_dmx_monitor_channel01_function(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"dmx.monitor.channel01.function","params":{"property":"dmx.monitor.channel01.function"}});

    session.call(payload).await
}
///Offset of the channel.
pub async fn get_dmx_monitor_channel01_offset(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"dmx.monitor.channel01.offset","params":{"property":"dmx.monitor.channel01.offset"}});

    session.call(payload).await
}
///Current dmx value for the channel
pub async fn get_dmx_monitor_channel01_value(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"dmx.monitor.channel01.value","params":{"property":"dmx.monitor.channel01.value"}});

    session.call(payload).await
}
///Description for the dmx channel
pub async fn get_dmx_monitor_channel02_function(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"dmx.monitor.channel02.function","params":{"property":"dmx.monitor.channel02.function"}});

    session.call(payload).await
}
///Offset of the channel.
pub async fn get_dmx_monitor_channel02_offset(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"dmx.monitor.channel02.offset","params":{"property":"dmx.monitor.channel02.offset"}});

    session.call(payload).await
}
///Current dmx value for the channel
pub async fn get_dmx_monitor_channel02_value(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"dmx.monitor.channel02.value","params":{"property":"dmx.monitor.channel02.value"}});

    session.call(payload).await
}
///true indicates that a dmx (if artnet setting is deactivated) or artnet package (if artnet setting is active) was received in the last 10 seconds.
pub async fn get_dmx_monitor_connectionstate_active(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"dmx.monitor.connectionstate.active","params":{"property":"dmx.monitor.connectionstate.active"}});

    session.call(payload).await
}
///Shutdown enabled or not
pub async fn set_dmx_shutdown(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "dmx.shutdown",
        "params": {
            "property": "dmx.shutdown",
            "value": value
        }
    });

    session.call(payload).await
}
///Time out for shutdown in minutes.
pub async fn set_dmx_shutdowntimeout(
    session: &mut PulseSession,
    value: u64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "dmx.shutdowntimeout",
        "params": {
            "property": "dmx.shutdowntimeout",
            "value": value
        }
    });

    session.call(payload).await
}
///The dmx start channel [1..512].
pub async fn set_dmx_startchannel(
    session: &mut PulseSession,
    value: u64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "dmx.startchannel",
        "params": {
            "property": "dmx.startchannel",
            "value": value
        }
    });

    session.call(payload).await
}
///[DEPRECATED] Replaced by 'system.health'
pub async fn get_environment_alarmstate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"environment.alarmstate","params":{"property":"environment.alarmstate"}});

    session.call(payload).await
}
///The firmware packages available for upgrade
pub async fn get_firmware_availablepackages(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.availablepackages","params":{"property":"firmware.availablepackages"}});

    session.call(payload).await
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_backplane_boot_redstar_actualversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.backplane-boot-redstar.actualversion","params":{"property":"firmware.component.backplane-boot-redstar.actualversion"}});

    session.call(payload).await
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_backplane_boot_redstar_displayname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.backplane-boot-redstar.displayname","params":{"property":"firmware.component.backplane-boot-redstar.displayname"}});

    session.call(payload).await
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_backplane_boot_redstar_error(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.backplane-boot-redstar.error","params":{"property":"firmware.component.backplane-boot-redstar.error"}});

    session.call(payload).await
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_backplane_boot_redstar_mismatch(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.backplane-boot-redstar.mismatch","params":{"property":"firmware.component.backplane-boot-redstar.mismatch"}});

    session.call(payload).await
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_backplane_boot_redstar_progress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.backplane-boot-redstar.progress","params":{"property":"firmware.component.backplane-boot-redstar.progress"}});

    session.call(payload).await
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_backplane_boot_redstar_requiredversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.backplane-boot-redstar.requiredversion","params":{"property":"firmware.component.backplane-boot-redstar.requiredversion"}});

    session.call(payload).await
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_backplane_boot_redstar_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.backplane-boot-redstar.status","params":{"property":"firmware.component.backplane-boot-redstar.status"}});

    session.call(payload).await
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_backplane_redstar_fpga_actualversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.backplane-redstar-fpga.actualversion","params":{"property":"firmware.component.backplane-redstar-fpga.actualversion"}});

    session.call(payload).await
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_backplane_redstar_fpga_displayname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.backplane-redstar-fpga.displayname","params":{"property":"firmware.component.backplane-redstar-fpga.displayname"}});

    session.call(payload).await
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_backplane_redstar_fpga_error(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.backplane-redstar-fpga.error","params":{"property":"firmware.component.backplane-redstar-fpga.error"}});

    session.call(payload).await
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_backplane_redstar_fpga_mismatch(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.backplane-redstar-fpga.mismatch","params":{"property":"firmware.component.backplane-redstar-fpga.mismatch"}});

    session.call(payload).await
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_backplane_redstar_fpga_progress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.backplane-redstar-fpga.progress","params":{"property":"firmware.component.backplane-redstar-fpga.progress"}});

    session.call(payload).await
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_backplane_redstar_fpga_requiredversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.backplane-redstar-fpga.requiredversion","params":{"property":"firmware.component.backplane-redstar-fpga.requiredversion"}});

    session.call(payload).await
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_backplane_redstar_fpga_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.backplane-redstar-fpga.status","params":{"property":"firmware.component.backplane-redstar-fpga.status"}});

    session.call(payload).await
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_backplane_run_redstar_actualversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.backplane-run-redstar.actualversion","params":{"property":"firmware.component.backplane-run-redstar.actualversion"}});

    session.call(payload).await
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_backplane_run_redstar_displayname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.backplane-run-redstar.displayname","params":{"property":"firmware.component.backplane-run-redstar.displayname"}});

    session.call(payload).await
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_backplane_run_redstar_error(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.backplane-run-redstar.error","params":{"property":"firmware.component.backplane-run-redstar.error"}});

    session.call(payload).await
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_backplane_run_redstar_mismatch(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.backplane-run-redstar.mismatch","params":{"property":"firmware.component.backplane-run-redstar.mismatch"}});

    session.call(payload).await
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_backplane_run_redstar_progress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.backplane-run-redstar.progress","params":{"property":"firmware.component.backplane-run-redstar.progress"}});

    session.call(payload).await
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_backplane_run_redstar_requiredversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.backplane-run-redstar.requiredversion","params":{"property":"firmware.component.backplane-run-redstar.requiredversion"}});

    session.call(payload).await
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_backplane_run_redstar_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.backplane-run-redstar.status","params":{"property":"firmware.component.backplane-run-redstar.status"}});

    session.call(payload).await
}
///List of components
pub async fn get_firmware_component_components(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.components","params":{"property":"firmware.component.components"}});

    session.call(payload).await
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_formatter_1dlp_fpga_actualversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-1dlp-fpga.actualversion","params":{"property":"firmware.component.formatter-1dlp-fpga.actualversion"}});

    session.call(payload).await
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_formatter_1dlp_fpga_displayname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-1dlp-fpga.displayname","params":{"property":"firmware.component.formatter-1dlp-fpga.displayname"}});

    session.call(payload).await
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_formatter_1dlp_fpga_error(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-1dlp-fpga.error","params":{"property":"firmware.component.formatter-1dlp-fpga.error"}});

    session.call(payload).await
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_formatter_1dlp_fpga_mismatch(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-1dlp-fpga.mismatch","params":{"property":"firmware.component.formatter-1dlp-fpga.mismatch"}});

    session.call(payload).await
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_formatter_1dlp_fpga_progress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-1dlp-fpga.progress","params":{"property":"firmware.component.formatter-1dlp-fpga.progress"}});

    session.call(payload).await
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_formatter_1dlp_fpga_requiredversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-1dlp-fpga.requiredversion","params":{"property":"firmware.component.formatter-1dlp-fpga.requiredversion"}});

    session.call(payload).await
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_formatter_1dlp_fpga_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-1dlp-fpga.status","params":{"property":"firmware.component.formatter-1dlp-fpga.status"}});

    session.call(payload).await
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_formatter_ddp442x_sequences_actualversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-ddp442x-sequences.actualversion","params":{"property":"firmware.component.formatter-ddp442x-sequences.actualversion"}});

    session.call(payload).await
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_formatter_ddp442x_sequences_displayname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-ddp442x-sequences.displayname","params":{"property":"firmware.component.formatter-ddp442x-sequences.displayname"}});

    session.call(payload).await
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_formatter_ddp442x_sequences_error(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-ddp442x-sequences.error","params":{"property":"firmware.component.formatter-ddp442x-sequences.error"}});

    session.call(payload).await
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_formatter_ddp442x_sequences_mismatch(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-ddp442x-sequences.mismatch","params":{"property":"firmware.component.formatter-ddp442x-sequences.mismatch"}});

    session.call(payload).await
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_formatter_ddp442x_sequences_progress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-ddp442x-sequences.progress","params":{"property":"firmware.component.formatter-ddp442x-sequences.progress"}});

    session.call(payload).await
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_formatter_ddp442x_sequences_requiredversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-ddp442x-sequences.requiredversion","params":{"property":"firmware.component.formatter-ddp442x-sequences.requiredversion"}});

    session.call(payload).await
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_formatter_ddp442x_sequences_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-ddp442x-sequences.status","params":{"property":"firmware.component.formatter-ddp442x-sequences.status"}});

    session.call(payload).await
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_formatter_rallar_boot_actualversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-rallar-boot.actualversion","params":{"property":"firmware.component.formatter-rallar-boot.actualversion"}});

    session.call(payload).await
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_formatter_rallar_boot_displayname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-rallar-boot.displayname","params":{"property":"firmware.component.formatter-rallar-boot.displayname"}});

    session.call(payload).await
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_formatter_rallar_boot_error(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-rallar-boot.error","params":{"property":"firmware.component.formatter-rallar-boot.error"}});

    session.call(payload).await
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_formatter_rallar_boot_mismatch(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-rallar-boot.mismatch","params":{"property":"firmware.component.formatter-rallar-boot.mismatch"}});

    session.call(payload).await
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_formatter_rallar_boot_progress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-rallar-boot.progress","params":{"property":"firmware.component.formatter-rallar-boot.progress"}});

    session.call(payload).await
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_formatter_rallar_boot_requiredversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-rallar-boot.requiredversion","params":{"property":"firmware.component.formatter-rallar-boot.requiredversion"}});

    session.call(payload).await
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_formatter_rallar_boot_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-rallar-boot.status","params":{"property":"firmware.component.formatter-rallar-boot.status"}});

    session.call(payload).await
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_formatter_rallar_run_actualversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-rallar-run.actualversion","params":{"property":"firmware.component.formatter-rallar-run.actualversion"}});

    session.call(payload).await
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_formatter_rallar_run_displayname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-rallar-run.displayname","params":{"property":"firmware.component.formatter-rallar-run.displayname"}});

    session.call(payload).await
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_formatter_rallar_run_error(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-rallar-run.error","params":{"property":"firmware.component.formatter-rallar-run.error"}});

    session.call(payload).await
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_formatter_rallar_run_mismatch(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-rallar-run.mismatch","params":{"property":"firmware.component.formatter-rallar-run.mismatch"}});

    session.call(payload).await
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_formatter_rallar_run_progress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-rallar-run.progress","params":{"property":"firmware.component.formatter-rallar-run.progress"}});

    session.call(payload).await
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_formatter_rallar_run_requiredversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-rallar-run.requiredversion","params":{"property":"firmware.component.formatter-rallar-run.requiredversion"}});

    session.call(payload).await
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_formatter_rallar_run_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.formatter-rallar-run.status","params":{"property":"firmware.component.formatter-rallar-run.status"}});

    session.call(payload).await
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_green_controller_boot_v1_actualversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.green-controller-boot-v1.actualversion","params":{"property":"firmware.component.green-controller-boot-v1.actualversion"}});

    session.call(payload).await
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_green_controller_boot_v1_displayname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.green-controller-boot-v1.displayname","params":{"property":"firmware.component.green-controller-boot-v1.displayname"}});

    session.call(payload).await
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_green_controller_boot_v1_error(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.green-controller-boot-v1.error","params":{"property":"firmware.component.green-controller-boot-v1.error"}});

    session.call(payload).await
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_green_controller_boot_v1_mismatch(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.green-controller-boot-v1.mismatch","params":{"property":"firmware.component.green-controller-boot-v1.mismatch"}});

    session.call(payload).await
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_green_controller_boot_v1_progress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.green-controller-boot-v1.progress","params":{"property":"firmware.component.green-controller-boot-v1.progress"}});

    session.call(payload).await
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_green_controller_boot_v1_requiredversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.green-controller-boot-v1.requiredversion","params":{"property":"firmware.component.green-controller-boot-v1.requiredversion"}});

    session.call(payload).await
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_green_controller_boot_v1_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.green-controller-boot-v1.status","params":{"property":"firmware.component.green-controller-boot-v1.status"}});

    session.call(payload).await
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_green_controller_run_v1_actualversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.green-controller-run-v1.actualversion","params":{"property":"firmware.component.green-controller-run-v1.actualversion"}});

    session.call(payload).await
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_green_controller_run_v1_displayname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.green-controller-run-v1.displayname","params":{"property":"firmware.component.green-controller-run-v1.displayname"}});

    session.call(payload).await
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_green_controller_run_v1_error(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.green-controller-run-v1.error","params":{"property":"firmware.component.green-controller-run-v1.error"}});

    session.call(payload).await
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_green_controller_run_v1_mismatch(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.green-controller-run-v1.mismatch","params":{"property":"firmware.component.green-controller-run-v1.mismatch"}});

    session.call(payload).await
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_green_controller_run_v1_progress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.green-controller-run-v1.progress","params":{"property":"firmware.component.green-controller-run-v1.progress"}});

    session.call(payload).await
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_green_controller_run_v1_requiredversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.green-controller-run-v1.requiredversion","params":{"property":"firmware.component.green-controller-run-v1.requiredversion"}});

    session.call(payload).await
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_green_controller_run_v1_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.green-controller-run-v1.status","params":{"property":"firmware.component.green-controller-run-v1.status"}});

    session.call(payload).await
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_ld1_megmeet_psu_actualversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.ld1-megmeet-psu.actualversion","params":{"property":"firmware.component.ld1-megmeet-psu.actualversion"}});

    session.call(payload).await
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_ld1_megmeet_psu_displayname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.ld1-megmeet-psu.displayname","params":{"property":"firmware.component.ld1-megmeet-psu.displayname"}});

    session.call(payload).await
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_ld1_megmeet_psu_error(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.ld1-megmeet-psu.error","params":{"property":"firmware.component.ld1-megmeet-psu.error"}});

    session.call(payload).await
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_ld1_megmeet_psu_mismatch(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.ld1-megmeet-psu.mismatch","params":{"property":"firmware.component.ld1-megmeet-psu.mismatch"}});

    session.call(payload).await
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_ld1_megmeet_psu_progress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.ld1-megmeet-psu.progress","params":{"property":"firmware.component.ld1-megmeet-psu.progress"}});

    session.call(payload).await
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_ld1_megmeet_psu_requiredversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.ld1-megmeet-psu.requiredversion","params":{"property":"firmware.component.ld1-megmeet-psu.requiredversion"}});

    session.call(payload).await
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_ld1_megmeet_psu_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.ld1-megmeet-psu.status","params":{"property":"firmware.component.ld1-megmeet-psu.status"}});

    session.call(payload).await
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_ld2_megmeet_psu_actualversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.ld2-megmeet-psu.actualversion","params":{"property":"firmware.component.ld2-megmeet-psu.actualversion"}});

    session.call(payload).await
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_ld2_megmeet_psu_displayname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.ld2-megmeet-psu.displayname","params":{"property":"firmware.component.ld2-megmeet-psu.displayname"}});

    session.call(payload).await
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_ld2_megmeet_psu_error(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.ld2-megmeet-psu.error","params":{"property":"firmware.component.ld2-megmeet-psu.error"}});

    session.call(payload).await
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_ld2_megmeet_psu_mismatch(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.ld2-megmeet-psu.mismatch","params":{"property":"firmware.component.ld2-megmeet-psu.mismatch"}});

    session.call(payload).await
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_ld2_megmeet_psu_progress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.ld2-megmeet-psu.progress","params":{"property":"firmware.component.ld2-megmeet-psu.progress"}});

    session.call(payload).await
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_ld2_megmeet_psu_requiredversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.ld2-megmeet-psu.requiredversion","params":{"property":"firmware.component.ld2-megmeet-psu.requiredversion"}});

    session.call(payload).await
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_ld2_megmeet_psu_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.ld2-megmeet-psu.status","params":{"property":"firmware.component.ld2-megmeet-psu.status"}});

    session.call(payload).await
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_linux_actualversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.linux.actualversion","params":{"property":"firmware.component.linux.actualversion"}});

    session.call(payload).await
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_linux_displayname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.linux.displayname","params":{"property":"firmware.component.linux.displayname"}});

    session.call(payload).await
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_linux_error(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.linux.error","params":{"property":"firmware.component.linux.error"}});

    session.call(payload).await
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_linux_mismatch(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.linux.mismatch","params":{"property":"firmware.component.linux.mismatch"}});

    session.call(payload).await
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_linux_progress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.linux.progress","params":{"property":"firmware.component.linux.progress"}});

    session.call(payload).await
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_linux_requiredversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.linux.requiredversion","params":{"property":"firmware.component.linux.requiredversion"}});

    session.call(payload).await
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_linux_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.linux.status","params":{"property":"firmware.component.linux.status"}});

    session.call(payload).await
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_boot_actualversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.mysticum-image-processing-3-mk22-boot.actualversion","params":{"property":"firmware.component.mysticum-image-processing-3-mk22-boot.actualversion"}});

    session.call(payload).await
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_boot_displayname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.mysticum-image-processing-3-mk22-boot.displayname","params":{"property":"firmware.component.mysticum-image-processing-3-mk22-boot.displayname"}});

    session.call(payload).await
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_boot_error(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.mysticum-image-processing-3-mk22-boot.error","params":{"property":"firmware.component.mysticum-image-processing-3-mk22-boot.error"}});

    session.call(payload).await
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_boot_mismatch(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.mysticum-image-processing-3-mk22-boot.mismatch","params":{"property":"firmware.component.mysticum-image-processing-3-mk22-boot.mismatch"}});

    session.call(payload).await
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_boot_progress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.mysticum-image-processing-3-mk22-boot.progress","params":{"property":"firmware.component.mysticum-image-processing-3-mk22-boot.progress"}});

    session.call(payload).await
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_boot_requiredversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.mysticum-image-processing-3-mk22-boot.requiredversion","params":{"property":"firmware.component.mysticum-image-processing-3-mk22-boot.requiredversion"}});

    session.call(payload).await
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_boot_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.mysticum-image-processing-3-mk22-boot.status","params":{"property":"firmware.component.mysticum-image-processing-3-mk22-boot.status"}});

    session.call(payload).await
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_fpga_actualversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.mysticum-image-processing-3-mk22-fpga.actualversion","params":{"property":"firmware.component.mysticum-image-processing-3-mk22-fpga.actualversion"}});

    session.call(payload).await
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_fpga_displayname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.mysticum-image-processing-3-mk22-fpga.displayname","params":{"property":"firmware.component.mysticum-image-processing-3-mk22-fpga.displayname"}});

    session.call(payload).await
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_fpga_error(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.mysticum-image-processing-3-mk22-fpga.error","params":{"property":"firmware.component.mysticum-image-processing-3-mk22-fpga.error"}});

    session.call(payload).await
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_fpga_mismatch(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.mysticum-image-processing-3-mk22-fpga.mismatch","params":{"property":"firmware.component.mysticum-image-processing-3-mk22-fpga.mismatch"}});

    session.call(payload).await
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_fpga_progress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.mysticum-image-processing-3-mk22-fpga.progress","params":{"property":"firmware.component.mysticum-image-processing-3-mk22-fpga.progress"}});

    session.call(payload).await
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_fpga_requiredversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.mysticum-image-processing-3-mk22-fpga.requiredversion","params":{"property":"firmware.component.mysticum-image-processing-3-mk22-fpga.requiredversion"}});

    session.call(payload).await
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_fpga_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.mysticum-image-processing-3-mk22-fpga.status","params":{"property":"firmware.component.mysticum-image-processing-3-mk22-fpga.status"}});

    session.call(payload).await
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_run_actualversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.mysticum-image-processing-3-mk22-run.actualversion","params":{"property":"firmware.component.mysticum-image-processing-3-mk22-run.actualversion"}});

    session.call(payload).await
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_run_displayname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.mysticum-image-processing-3-mk22-run.displayname","params":{"property":"firmware.component.mysticum-image-processing-3-mk22-run.displayname"}});

    session.call(payload).await
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_run_error(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.mysticum-image-processing-3-mk22-run.error","params":{"property":"firmware.component.mysticum-image-processing-3-mk22-run.error"}});

    session.call(payload).await
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_run_mismatch(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.mysticum-image-processing-3-mk22-run.mismatch","params":{"property":"firmware.component.mysticum-image-processing-3-mk22-run.mismatch"}});

    session.call(payload).await
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_run_progress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.mysticum-image-processing-3-mk22-run.progress","params":{"property":"firmware.component.mysticum-image-processing-3-mk22-run.progress"}});

    session.call(payload).await
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_run_requiredversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.mysticum-image-processing-3-mk22-run.requiredversion","params":{"property":"firmware.component.mysticum-image-processing-3-mk22-run.requiredversion"}});

    session.call(payload).await
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_mysticum_image_processing_3_mk22_run_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.mysticum-image-processing-3-mk22-run.status","params":{"property":"firmware.component.mysticum-image-processing-3-mk22-run.status"}});

    session.call(payload).await
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_redstar_fan_motor_control_board_fpga_actualversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.redstar-fan-motor-control-board-fpga.actualversion","params":{"property":"firmware.component.redstar-fan-motor-control-board-fpga.actualversion"}});

    session.call(payload).await
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_redstar_fan_motor_control_board_fpga_displayname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.redstar-fan-motor-control-board-fpga.displayname","params":{"property":"firmware.component.redstar-fan-motor-control-board-fpga.displayname"}});

    session.call(payload).await
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_redstar_fan_motor_control_board_fpga_error(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.redstar-fan-motor-control-board-fpga.error","params":{"property":"firmware.component.redstar-fan-motor-control-board-fpga.error"}});

    session.call(payload).await
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_redstar_fan_motor_control_board_fpga_mismatch(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.redstar-fan-motor-control-board-fpga.mismatch","params":{"property":"firmware.component.redstar-fan-motor-control-board-fpga.mismatch"}});

    session.call(payload).await
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_redstar_fan_motor_control_board_fpga_progress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.redstar-fan-motor-control-board-fpga.progress","params":{"property":"firmware.component.redstar-fan-motor-control-board-fpga.progress"}});

    session.call(payload).await
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_redstar_fan_motor_control_board_fpga_requiredversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.redstar-fan-motor-control-board-fpga.requiredversion","params":{"property":"firmware.component.redstar-fan-motor-control-board-fpga.requiredversion"}});

    session.call(payload).await
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_redstar_fan_motor_control_board_fpga_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.redstar-fan-motor-control-board-fpga.status","params":{"property":"firmware.component.redstar-fan-motor-control-board-fpga.status"}});

    session.call(payload).await
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_redstar_fan_motor_control_boot_actualversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.redstar-fan-motor-control-boot.actualversion","params":{"property":"firmware.component.redstar-fan-motor-control-boot.actualversion"}});

    session.call(payload).await
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_redstar_fan_motor_control_boot_displayname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.redstar-fan-motor-control-boot.displayname","params":{"property":"firmware.component.redstar-fan-motor-control-boot.displayname"}});

    session.call(payload).await
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_redstar_fan_motor_control_boot_error(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.redstar-fan-motor-control-boot.error","params":{"property":"firmware.component.redstar-fan-motor-control-boot.error"}});

    session.call(payload).await
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_redstar_fan_motor_control_boot_mismatch(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.redstar-fan-motor-control-boot.mismatch","params":{"property":"firmware.component.redstar-fan-motor-control-boot.mismatch"}});

    session.call(payload).await
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_redstar_fan_motor_control_boot_progress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.redstar-fan-motor-control-boot.progress","params":{"property":"firmware.component.redstar-fan-motor-control-boot.progress"}});

    session.call(payload).await
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_redstar_fan_motor_control_boot_requiredversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.redstar-fan-motor-control-boot.requiredversion","params":{"property":"firmware.component.redstar-fan-motor-control-boot.requiredversion"}});

    session.call(payload).await
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_redstar_fan_motor_control_boot_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.redstar-fan-motor-control-boot.status","params":{"property":"firmware.component.redstar-fan-motor-control-boot.status"}});

    session.call(payload).await
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_redstar_fan_motor_control_run_actualversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.redstar-fan-motor-control-run.actualversion","params":{"property":"firmware.component.redstar-fan-motor-control-run.actualversion"}});

    session.call(payload).await
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_redstar_fan_motor_control_run_displayname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.redstar-fan-motor-control-run.displayname","params":{"property":"firmware.component.redstar-fan-motor-control-run.displayname"}});

    session.call(payload).await
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_redstar_fan_motor_control_run_error(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.redstar-fan-motor-control-run.error","params":{"property":"firmware.component.redstar-fan-motor-control-run.error"}});

    session.call(payload).await
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_redstar_fan_motor_control_run_mismatch(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.redstar-fan-motor-control-run.mismatch","params":{"property":"firmware.component.redstar-fan-motor-control-run.mismatch"}});

    session.call(payload).await
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_redstar_fan_motor_control_run_progress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.redstar-fan-motor-control-run.progress","params":{"property":"firmware.component.redstar-fan-motor-control-run.progress"}});

    session.call(payload).await
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_redstar_fan_motor_control_run_requiredversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.redstar-fan-motor-control-run.requiredversion","params":{"property":"firmware.component.redstar-fan-motor-control-run.requiredversion"}});

    session.call(payload).await
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_redstar_fan_motor_control_run_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.redstar-fan-motor-control-run.status","params":{"property":"firmware.component.redstar-fan-motor-control-run.status"}});

    session.call(payload).await
}
///The status of the current component firmware upgrade
pub async fn get_firmware_component_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.status","params":{"property":"firmware.component.status"}});

    session.call(payload).await
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_u_boot_actualversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.u-boot.actualversion","params":{"property":"firmware.component.u-boot.actualversion"}});

    session.call(payload).await
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_u_boot_displayname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.u-boot.displayname","params":{"property":"firmware.component.u-boot.displayname"}});

    session.call(payload).await
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_u_boot_error(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.u-boot.error","params":{"property":"firmware.component.u-boot.error"}});

    session.call(payload).await
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_u_boot_mismatch(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.u-boot.mismatch","params":{"property":"firmware.component.u-boot.mismatch"}});

    session.call(payload).await
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_u_boot_progress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.u-boot.progress","params":{"property":"firmware.component.u-boot.progress"}});

    session.call(payload).await
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_u_boot_requiredversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.u-boot.requiredversion","params":{"property":"firmware.component.u-boot.requiredversion"}});

    session.call(payload).await
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_u_boot_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.u-boot.status","params":{"property":"firmware.component.u-boot.status"}});

    session.call(payload).await
}
///The version of the currently installed firmware.
pub async fn get_firmware_component_zynq_fpga_actualversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.zynq-fpga.actualversion","params":{"property":"firmware.component.zynq-fpga.actualversion"}});

    session.call(payload).await
}
///The user-friendly name of the firmware component.
pub async fn get_firmware_component_zynq_fpga_displayname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.zynq-fpga.displayname","params":{"property":"firmware.component.zynq-fpga.displayname"}});

    session.call(payload).await
}
///The error message in case of an upgrade error.
pub async fn get_firmware_component_zynq_fpga_error(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.zynq-fpga.error","params":{"property":"firmware.component.zynq-fpga.error"}});

    session.call(payload).await
}
///Indicate whether there is a version mismatch
pub async fn get_firmware_component_zynq_fpga_mismatch(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.zynq-fpga.mismatch","params":{"property":"firmware.component.zynq-fpga.mismatch"}});

    session.call(payload).await
}
///The progress of the current firmware upgrade
pub async fn get_firmware_component_zynq_fpga_progress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.zynq-fpga.progress","params":{"property":"firmware.component.zynq-fpga.progress"}});

    session.call(payload).await
}
///The firmware version this component is expected to have by the current firmware package.
pub async fn get_firmware_component_zynq_fpga_requiredversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.zynq-fpga.requiredversion","params":{"property":"firmware.component.zynq-fpga.requiredversion"}});

    session.call(payload).await
}
///The status of the current firmware upgrade
pub async fn get_firmware_component_zynq_fpga_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.component.zynq-fpga.status","params":{"property":"firmware.component.zynq-fpga.status"}});

    session.call(payload).await
}
///The error of the last firmware upgrade (only set if status is error)
pub async fn get_firmware_error(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.error","params":{"property":"firmware.error"}});

    session.call(payload).await
}
///Indiates whether or not the user is required to accept the term agreement.
pub async fn get_firmware_eula_requiretermagreement(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.eula.requiretermagreement","params":{"property":"firmware.eula.requiretermagreement"}});

    session.call(payload).await
}
///The user agreement with the firmware EULA.
pub async fn set_firmware_eula_termagreement(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "firmware.eula.termagreement",
        "params": {
            "property": "firmware.eula.termagreement",
            "value": value
        }
    });

    session.call(payload).await
}
///The version of the currently installed firmware.
pub async fn get_firmware_firmwareversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.firmwareversion","params":{"property":"firmware.firmwareversion"}});

    session.call(payload).await
}
///The progress of the current firmware upgrade
pub async fn get_firmware_progress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.progress","params":{"property":"firmware.progress"}});

    session.call(payload).await
}
///The status of the current firmware upgrade
pub async fn get_firmware_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"firmware.status","params":{"property":"firmware.status"}});

    session.call(payload).await
}
///Description of gpo
pub async fn get_gpio_gpo_trigger1_description(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"gpio.gpo.trigger1.description","params":{"property":"gpio.gpo.trigger1.description"}});

    session.call(payload).await
}
///Mode of output, level or pulsetrain
pub async fn set_gpio_gpo_trigger1_mode(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "gpio.gpo.trigger1.mode",
        "params": {
            "property": "gpio.gpo.trigger1.mode",
            "value": value
        }
    });

    session.call(payload).await
}
///Name of gpo
pub async fn get_gpio_gpo_trigger1_name(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"gpio.gpo.trigger1.name","params":{"property":"gpio.gpo.trigger1.name"}});

    session.call(payload).await
}
///Is GPIO reserved
pub async fn get_gpio_gpo_trigger1_reserved(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"gpio.gpo.trigger1.reserved","params":{"property":"gpio.gpo.trigger1.reserved"}});

    session.call(payload).await
}
///Value of output
pub async fn set_gpio_gpo_trigger1_value(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "gpio.gpo.trigger1.value",
        "params": {
            "property": "gpio.gpo.trigger1.value",
            "value": value
        }
    });

    session.call(payload).await
}
///Description of gpo
pub async fn get_gpio_gpo_trigger2_description(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"gpio.gpo.trigger2.description","params":{"property":"gpio.gpo.trigger2.description"}});

    session.call(payload).await
}
///Mode of output, level or pulsetrain
pub async fn set_gpio_gpo_trigger2_mode(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "gpio.gpo.trigger2.mode",
        "params": {
            "property": "gpio.gpo.trigger2.mode",
            "value": value
        }
    });

    session.call(payload).await
}
///Name of gpo
pub async fn get_gpio_gpo_trigger2_name(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"gpio.gpo.trigger2.name","params":{"property":"gpio.gpo.trigger2.name"}});

    session.call(payload).await
}
///Is GPIO reserved
pub async fn get_gpio_gpo_trigger2_reserved(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"gpio.gpo.trigger2.reserved","params":{"property":"gpio.gpo.trigger2.reserved"}});

    session.call(payload).await
}
///Value of output
pub async fn set_gpio_gpo_trigger2_value(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "gpio.gpo.trigger2.value",
        "params": {
            "property": "gpio.gpo.trigger2.value",
            "value": value
        }
    });

    session.call(payload).await
}
///The current status of the mobile connection.
pub async fn get_gsm_data_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"gsm.data.status","params":{"property":"gsm.data.status"}});

    session.call(payload).await
}
///The ICCID that identifies the SIM.
pub async fn get_gsm_iccid(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"gsm.iccid","params":{"property":"gsm.iccid"}});

    session.call(payload).await
}
///The IMEI that identifies the GSM module.
pub async fn get_gsm_imei(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"gsm.imei","params":{"property":"gsm.imei"}});

    session.call(payload).await
}
///The signal strength in percent.
pub async fn get_gsm_signalstrength(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"gsm.signalstrength","params":{"property":"gsm.signalstrength"}});

    session.call(payload).await
}
///Indicates which features are preventing CLO from being enabled
pub async fn get_illumination_clo_allowed(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"illumination.clo.allowed","params":{"property":"illumination.clo.allowed"}});

    session.call(payload).await
}
///Shows the current availability.
pub async fn get_illumination_clo_availability(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"illumination.clo.availability","params":{"property":"illumination.clo.availability"}});

    session.call(payload).await
}
///True if constant light output is enabled, false otherwise
pub async fn set_illumination_clo_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "illumination.clo.enable",
        "params": {
            "property": "illumination.clo.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///The percentage to scale the setpoint by.
pub async fn set_illumination_clo_scale(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "illumination.clo.scale",
        "params": {
            "property": "illumination.clo.scale",
            "value": value
        }
    });

    session.call(payload).await
}
///The target luminosity of the light source
pub async fn set_illumination_clo_setpoint(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "illumination.clo.setpoint",
        "params": {
            "property": "illumination.clo.setpoint",
            "value": value
        }
    });

    session.call(payload).await
}
///State of the CLO
pub async fn get_illumination_clo_state(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"illumination.clo.state","params":{"property":"illumination.clo.state"}});

    session.call(payload).await
}
///Enable/disable NoSignal dimming of the power
pub async fn set_illumination_nosignal_dimming_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "illumination.nosignal.dimming.enable",
        "params": {
            "property": "illumination.nosignal.dimming.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///The power limitation (in percent) that the NoSignal dimming will apply
pub async fn get_illumination_nosignal_dimming_power(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"illumination.nosignal.dimming.power","params":{"property":"illumination.nosignal.dimming.power"}});

    session.call(payload).await
}
///actual power in percent with limits
pub async fn get_illumination_sources_laser_actualpower(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"illumination.sources.laser.actualpower","params":{"property":"illumination.sources.laser.actualpower"}});

    session.call(payload).await
}
///Describes whether CLO is controlling this source.
pub async fn get_illumination_sources_laser_controlledbyclo(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"illumination.sources.laser.controlledbyclo","params":{"property":"illumination.sources.laser.controlledbyclo"}});

    session.call(payload).await
}
///Whether power is currently limited.
pub async fn get_illumination_sources_laser_ispowerlimited(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"illumination.sources.laser.ispowerlimited","params":{"property":"illumination.sources.laser.ispowerlimited"}});

    session.call(payload).await
}
///maximum power in percent
pub async fn get_illumination_sources_laser_maxpower(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"illumination.sources.laser.maxpower","params":{"property":"illumination.sources.laser.maxpower"}});

    session.call(payload).await
}
///minimum power in percent
pub async fn get_illumination_sources_laser_minpower(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"illumination.sources.laser.minpower","params":{"property":"illumination.sources.laser.minpower"}});

    session.call(payload).await
}
///target power in percent
pub async fn set_illumination_sources_laser_power(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "illumination.sources.laser.power",
        "params": {
            "property": "illumination.sources.laser.power",
            "value": value
        }
    });

    session.call(payload).await
}
///Contains the reason of the authority that limited the power.
pub async fn get_illumination_sources_laser_powerlimitreason(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"illumination.sources.laser.powerlimitreason","params":{"property":"illumination.sources.laser.powerlimitreason"}});

    session.call(payload).await
}
///Power precision this source takes into account, expressed as number of digits to the right of the decimal point.
pub async fn get_illumination_sources_laser_powerprecision(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"illumination.sources.laser.powerprecision","params":{"property":"illumination.sources.laser.powerprecision"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_illumination_sources_laser_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"illumination.sources.laser.status","params":{"property":"illumination.sources.laser.status"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_illumination_sources_laser_timcuring_dmd_cureavailable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"illumination.sources.laser.timcuring.dmd.cureavailable","params":{"property":"illumination.sources.laser.timcuring.dmd.cureavailable"}});

    session.call(payload).await
}
///State of the current or past curing execution expressed in the format of a date, a status and a list of failed plates
pub async fn get_illumination_sources_laser_timcuring_dmd_state(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"illumination.sources.laser.timcuring.dmd.state","params":{"property":"illumination.sources.laser.timcuring.dmd.state"}});

    session.call(payload).await
}
///The state of light
pub async fn get_illumination_state(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"illumination.state","params":{"property":"illumination.state"}});

    session.call(payload).await
}
///Image brightness/offset. The value is normalized, 0 is default, 1 is 100% offset.
pub async fn set_image_brightness(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.brightness",
        "params": {
            "property": "image.brightness",
            "value": value
        }
    });

    session.call(payload).await
}
///The current BrilliantColor mode.
pub async fn set_image_brilliantcolor_mode(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.brilliantcolor.mode",
        "params": {
            "property": "image.brilliantcolor.mode",
            "value": value
        }
    });

    session.call(payload).await
}
///Desired blue gain value
pub async fn set_image_color_p7_custom_bluegain(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.bluegain",
        "params": {
            "property": "image.color.p7.custom.bluegain",
            "value": value
        }
    });

    session.call(payload).await
}
///Desired blue luminanace
pub async fn set_image_color_p7_custom_bluelum(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.bluelum",
        "params": {
            "property": "image.color.p7.custom.bluelum",
            "value": value
        }
    });

    session.call(payload).await
}
///Desired blue x in xy-coordinates
pub async fn set_image_color_p7_custom_bluex(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.bluex",
        "params": {
            "property": "image.color.p7.custom.bluex",
            "value": value
        }
    });

    session.call(payload).await
}
///Desired blue y in xy-coordinates
pub async fn set_image_color_p7_custom_bluey(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.bluey",
        "params": {
            "property": "image.color.p7.custom.bluey",
            "value": value
        }
    });

    session.call(payload).await
}
///true if secondaries should be shown (OSD)
pub async fn get_image_color_p7_custom_cmyreadable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.custom.cmyreadable","params":{"property":"image.color.p7.custom.cmyreadable"}});

    session.call(payload).await
}
///true if secondaries are Writable
pub async fn get_image_color_p7_custom_cmywritable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.custom.cmywritable","params":{"property":"image.color.p7.custom.cmywritable"}});

    session.call(payload).await
}
///Desired cyan gain value
pub async fn set_image_color_p7_custom_cyangain(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.cyangain",
        "params": {
            "property": "image.color.p7.custom.cyangain",
            "value": value
        }
    });

    session.call(payload).await
}
///Desired cyan luminanace
pub async fn set_image_color_p7_custom_cyanlum(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.cyanlum",
        "params": {
            "property": "image.color.p7.custom.cyanlum",
            "value": value
        }
    });

    session.call(payload).await
}
///Desired cyan x in xy-coordinates
pub async fn set_image_color_p7_custom_cyanx(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.cyanx",
        "params": {
            "property": "image.color.p7.custom.cyanx",
            "value": value
        }
    });

    session.call(payload).await
}
///Desired cyan y in xy-coordinates
pub async fn set_image_color_p7_custom_cyany(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.cyany",
        "params": {
            "property": "image.color.p7.custom.cyany",
            "value": value
        }
    });

    session.call(payload).await
}
///true when gains are available
pub async fn get_image_color_p7_custom_gainsavailable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.custom.gainsavailable","params":{"property":"image.color.p7.custom.gainsavailable"}});

    session.call(payload).await
}
///Desired green gain value
pub async fn set_image_color_p7_custom_greengain(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.greengain",
        "params": {
            "property": "image.color.p7.custom.greengain",
            "value": value
        }
    });

    session.call(payload).await
}
///Desired green luminanace
pub async fn set_image_color_p7_custom_greenlum(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.greenlum",
        "params": {
            "property": "image.color.p7.custom.greenlum",
            "value": value
        }
    });

    session.call(payload).await
}
///Desired green x in xy-coordinates
pub async fn set_image_color_p7_custom_greenx(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.greenx",
        "params": {
            "property": "image.color.p7.custom.greenx",
            "value": value
        }
    });

    session.call(payload).await
}
///Desired green y in xy-coordinates
pub async fn set_image_color_p7_custom_greeny(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.greeny",
        "params": {
            "property": "image.color.p7.custom.greeny",
            "value": value
        }
    });

    session.call(payload).await
}
///true if luminances are available
pub async fn get_image_color_p7_custom_luminancesavailable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.custom.luminancesavailable","params":{"property":"image.color.p7.custom.luminancesavailable"}});

    session.call(payload).await
}
///Desired magenta gain value
pub async fn set_image_color_p7_custom_magentagain(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.magentagain",
        "params": {
            "property": "image.color.p7.custom.magentagain",
            "value": value
        }
    });

    session.call(payload).await
}
///Desired magenta luminanace
pub async fn set_image_color_p7_custom_magentalum(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.magentalum",
        "params": {
            "property": "image.color.p7.custom.magentalum",
            "value": value
        }
    });

    session.call(payload).await
}
///Desired magenta x in xy-coordinates
pub async fn set_image_color_p7_custom_magentax(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.magentax",
        "params": {
            "property": "image.color.p7.custom.magentax",
            "value": value
        }
    });

    session.call(payload).await
}
///Desired magenta y in xy-coordinates
pub async fn set_image_color_p7_custom_magentay(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.magentay",
        "params": {
            "property": "image.color.p7.custom.magentay",
            "value": value
        }
    });

    session.call(payload).await
}
///Description not provided
pub async fn set_image_color_p7_custom_mode(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.mode",
        "params": {
            "property": "image.color.p7.custom.mode",
            "value": value
        }
    });

    session.call(payload).await
}
///Description not provided
pub async fn get_image_color_p7_custom_modes(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.custom.modes","params":{"property":"image.color.p7.custom.modes"}});

    session.call(payload).await
}
///Desired red gain value
pub async fn set_image_color_p7_custom_redgain(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.redgain",
        "params": {
            "property": "image.color.p7.custom.redgain",
            "value": value
        }
    });

    session.call(payload).await
}
///Desired red luminanace
pub async fn set_image_color_p7_custom_redlum(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.redlum",
        "params": {
            "property": "image.color.p7.custom.redlum",
            "value": value
        }
    });

    session.call(payload).await
}
///Desired red x in xy-coordinates
pub async fn set_image_color_p7_custom_redx(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.redx",
        "params": {
            "property": "image.color.p7.custom.redx",
            "value": value
        }
    });

    session.call(payload).await
}
///Desired red y in xy-coordinates
pub async fn set_image_color_p7_custom_redy(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.redy",
        "params": {
            "property": "image.color.p7.custom.redy",
            "value": value
        }
    });

    session.call(payload).await
}
///true when R,G,B,C,M,Y gains are available
pub async fn get_image_color_p7_custom_rgbcmygainsavailable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.custom.rgbcmygainsavailable","params":{"property":"image.color.p7.custom.rgbcmygainsavailable"}});

    session.call(payload).await
}
///true if RGB are Writable (not in WHITE mode)
pub async fn get_image_color_p7_custom_rgbwritable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.custom.rgbwritable","params":{"property":"image.color.p7.custom.rgbwritable"}});

    session.call(payload).await
}
///Target color coordinates for the current preset
pub async fn get_image_color_p7_custom_target(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.custom.target","params":{"property":"image.color.p7.custom.target"}});

    session.call(payload).await
}
///Desired white gain value
pub async fn set_image_color_p7_custom_whitegain(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.whitegain",
        "params": {
            "property": "image.color.p7.custom.whitegain",
            "value": value
        }
    });

    session.call(payload).await
}
///true when white gain is available
pub async fn get_image_color_p7_custom_whitegainavailable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.custom.whitegainavailable","params":{"property":"image.color.p7.custom.whitegainavailable"}});

    session.call(payload).await
}
///Desired white luminanace
pub async fn set_image_color_p7_custom_whitelum(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.whitelum",
        "params": {
            "property": "image.color.p7.custom.whitelum",
            "value": value
        }
    });

    session.call(payload).await
}
///Description not provided
pub async fn set_image_color_p7_custom_whitemode(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.whitemode",
        "params": {
            "property": "image.color.p7.custom.whitemode",
            "value": value
        }
    });

    session.call(payload).await
}
///Desired white point temperature
pub async fn set_image_color_p7_custom_whitetemperature(
    session: &mut PulseSession,
    value: u64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.whitetemperature",
        "params": {
            "property": "image.color.p7.custom.whitetemperature",
            "value": value
        }
    });

    session.call(payload).await
}
///true if White temperature is available
pub async fn get_image_color_p7_custom_whitetemperatureavailable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.custom.whitetemperatureavailable","params":{"property":"image.color.p7.custom.whitetemperatureavailable"}});

    session.call(payload).await
}
///true if White is Writable
pub async fn get_image_color_p7_custom_whitewritable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.custom.whitewritable","params":{"property":"image.color.p7.custom.whitewritable"}});

    session.call(payload).await
}
///Desired white x in xy-coordinates
pub async fn set_image_color_p7_custom_whitex(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.whitex",
        "params": {
            "property": "image.color.p7.custom.whitex",
            "value": value
        }
    });

    session.call(payload).await
}
///Desired white y in xy-coordinates
pub async fn set_image_color_p7_custom_whitey(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.whitey",
        "params": {
            "property": "image.color.p7.custom.whitey",
            "value": value
        }
    });

    session.call(payload).await
}
///Desired yellow gain value
pub async fn set_image_color_p7_custom_yellowgain(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.yellowgain",
        "params": {
            "property": "image.color.p7.custom.yellowgain",
            "value": value
        }
    });

    session.call(payload).await
}
///Desired yellow luminanace
pub async fn set_image_color_p7_custom_yellowlum(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.yellowlum",
        "params": {
            "property": "image.color.p7.custom.yellowlum",
            "value": value
        }
    });

    session.call(payload).await
}
///Desired yellow x in xy-coordinates
pub async fn set_image_color_p7_custom_yellowx(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.yellowx",
        "params": {
            "property": "image.color.p7.custom.yellowx",
            "value": value
        }
    });

    session.call(payload).await
}
///Desired yellow y in xy-coordinates
pub async fn set_image_color_p7_custom_yellowy(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.p7.custom.yellowy",
        "params": {
            "property": "image.color.p7.custom.yellowy",
            "value": value
        }
    });

    session.call(payload).await
}
///Native C1 x in xy-coordinates
pub async fn get_image_color_p7_native_c1(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.native.c1","params":{"property":"image.color.p7.native.c1"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_image_color_p7_native_c1available(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.native.c1available","params":{"property":"image.color.p7.native.c1available"}});

    session.call(payload).await
}
///Native C2 x in xy-coordinates
pub async fn get_image_color_p7_native_c2(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.native.c2","params":{"property":"image.color.p7.native.c2"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_image_color_p7_native_c2available(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.native.c2available","params":{"property":"image.color.p7.native.c2available"}});

    session.call(payload).await
}
///list available native sets
pub async fn get_image_color_p7_native_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.native.list","params":{"property":"image.color.p7.native.list"}});

    session.call(payload).await
}
///Native C1 x in xy-coordinates
pub async fn get_image_color_p7_native_normal_c1(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.native.normal.c1","params":{"property":"image.color.p7.native.normal.c1"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_image_color_p7_native_normal_c1available(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.native.normal.c1available","params":{"property":"image.color.p7.native.normal.c1available"}});

    session.call(payload).await
}
///Native C2 x in xy-coordinates
pub async fn get_image_color_p7_native_normal_c2(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.native.normal.c2","params":{"property":"image.color.p7.native.normal.c2"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_image_color_p7_native_normal_c2available(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.native.normal.c2available","params":{"property":"image.color.p7.native.normal.c2available"}});

    session.call(payload).await
}
///Native red x in xy-coordinates
pub async fn get_image_color_p7_native_normal_rgbw(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.native.normal.rgbw","params":{"property":"image.color.p7.native.normal.rgbw"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_image_color_p7_native_normal_whiteavailable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.native.normal.whiteavailable","params":{"property":"image.color.p7.native.normal.whiteavailable"}});

    session.call(payload).await
}
///Native red x in xy-coordinates
pub async fn get_image_color_p7_native_rgbw(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.native.rgbw","params":{"property":"image.color.p7.native.rgbw"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_image_color_p7_native_whiteavailable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.color.p7.native.whiteavailable","params":{"property":"image.color.p7.native.whiteavailable"}});

    session.call(payload).await
}
///RGB Mode
pub async fn set_image_color_rgbmode_rgbmode(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.color.rgbmode.rgbmode",
        "params": {
            "property": "image.color.rgbmode.rgbmode",
            "value": value
        }
    });

    session.call(payload).await
}
///Capabilities.
pub async fn get_image_connector_displayport_capabilities(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.connector.displayport.capabilities","params":{"property":"image.connector.displayport.capabilities"}});

    session.call(payload).await
}
///Override the detected signal color primaries. Set to Auto for automatic control.
pub async fn set_image_connector_displayport_colorprimaries(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.connector.displayport.colorprimaries",
        "params": {
            "property": "image.connector.displayport.colorprimaries",
            "value": value
        }
    });

    session.call(payload).await
}
///true if Color Primaries is available
pub async fn get_image_connector_displayport_colorprimariesavailable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.connector.displayport.colorprimariesavailable","params":{"property":"image.connector.displayport.colorprimariesavailable"}});

    session.call(payload).await
}
///Override the detected signal color space. Set to Auto for automatic control.
pub async fn set_image_connector_displayport_colorspace(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.connector.displayport.colorspace",
        "params": {
            "property": "image.connector.displayport.colorspace",
            "value": value
        }
    });

    session.call(payload).await
}
///The signal information of the currently detected signal. If 'active' is false, there is no detected signal and the rest of the information should be disregarded. is_stereo indicates if stereo_mode is different from none.
pub async fn get_image_connector_displayport_detectedsignal(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.connector.displayport.detectedsignal","params":{"property":"image.connector.displayport.detectedsignal"}});

    session.call(payload).await
}
///Selected EDID for connector
pub async fn set_image_connector_displayport_edid_selected(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.connector.displayport.edid.selected",
        "params": {
            "property": "image.connector.displayport.edid.selected",
            "value": value
        }
    });

    session.call(payload).await
}
///Name of the source
pub async fn get_image_connector_displayport_name(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.connector.displayport.name","params":{"property":"image.connector.displayport.name"}});

    session.call(payload).await
}
///Override the detected signal signal range. Set to Auto for automatic control.
pub async fn set_image_connector_displayport_signalrange(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.connector.displayport.signalrange",
        "params": {
            "property": "image.connector.displayport.signalrange",
            "value": value
        }
    });

    session.call(payload).await
}
///Capabilities.
pub async fn get_image_connector_hdmi_capabilities(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.connector.hdmi.capabilities","params":{"property":"image.connector.hdmi.capabilities"}});

    session.call(payload).await
}
///Override the detected signal color primaries. Set to Auto for automatic control.
pub async fn set_image_connector_hdmi_colorprimaries(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.connector.hdmi.colorprimaries",
        "params": {
            "property": "image.connector.hdmi.colorprimaries",
            "value": value
        }
    });

    session.call(payload).await
}
///true if Color Primaries is available
pub async fn get_image_connector_hdmi_colorprimariesavailable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.connector.hdmi.colorprimariesavailable","params":{"property":"image.connector.hdmi.colorprimariesavailable"}});

    session.call(payload).await
}
///Override the detected signal color space. Set to Auto for automatic control.
pub async fn set_image_connector_hdmi_colorspace(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.connector.hdmi.colorspace",
        "params": {
            "property": "image.connector.hdmi.colorspace",
            "value": value
        }
    });

    session.call(payload).await
}
///The signal information of the currently detected signal. If 'active' is false, there is no detected signal and the rest of the information should be disregarded. is_stereo indicates if stereo_mode is different from none.
pub async fn get_image_connector_hdmi_detectedsignal(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.connector.hdmi.detectedsignal","params":{"property":"image.connector.hdmi.detectedsignal"}});

    session.call(payload).await
}
///Selected EDID for connector
pub async fn set_image_connector_hdmi_edid_selected(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.connector.hdmi.edid.selected",
        "params": {
            "property": "image.connector.hdmi.edid.selected",
            "value": value
        }
    });

    session.call(payload).await
}
///Name of the source
pub async fn get_image_connector_hdmi_name(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.connector.hdmi.name","params":{"property":"image.connector.hdmi.name"}});

    session.call(payload).await
}
///Override the detected signal signal range. Set to Auto for automatic control.
pub async fn set_image_connector_hdmi_signalrange(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.connector.hdmi.signalrange",
        "params": {
            "property": "image.connector.hdmi.signalrange",
            "value": value
        }
    });

    session.call(payload).await
}
///Image contrast/gain. The value is normalized, 1 is default.
pub async fn set_image_contrast(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.contrast",
        "params": {
            "property": "image.contrast",
            "value": value
        }
    });

    session.call(payload).await
}
///The desired display mode.
pub async fn set_image_display_desireddisplaymode(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.display.desireddisplaymode",
        "params": {
            "property": "image.display.desireddisplaymode",
            "value": value
        }
    });

    session.call(payload).await
}
///The current display mode.
pub async fn get_image_display_displaymode(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.display.displaymode","params":{"property":"image.display.displaymode"}});

    session.call(payload).await
}
///The display frequency.
pub async fn get_image_display_frequency(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.display.frequency","params":{"property":"image.display.frequency"}});

    session.call(payload).await
}
///The display synchronous lock state.
pub async fn get_image_display_synchronouslock(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.display.synchronouslock","params":{"property":"image.display.synchronouslock"}});

    session.call(payload).await
}
///Amount of dynamic contrast that will be applied [0:4]
pub async fn set_image_dynamiccontrast_level(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.dynamiccontrast.level",
        "params": {
            "property": "image.dynamiccontrast.level",
            "value": value
        }
    });

    session.call(payload).await
}
///Image gamma. Default is 2.2
pub async fn set_image_gamma(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.gamma",
        "params": {
            "property": "image.gamma",
            "value": value
        }
    });

    session.call(payload).await
}
///Override the detected signal gamma type. Set to Auto for automatic control.
pub async fn set_image_gammatype(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.gammatype",
        "params": {
            "property": "image.gammatype",
            "value": value
        }
    });

    session.call(payload).await
}
///Intensity
pub async fn set_image_intensity(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.intensity",
        "params": {
            "property": "image.intensity",
            "value": value
        }
    });

    session.call(payload).await
}
///Description not provided
pub async fn set_image_orientation(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.orientation",
        "params": {
            "property": "image.orientation",
            "value": value
        }
    });

    session.call(payload).await
}
///The mode of orientation as auto detected or a manual fixed setting
pub async fn set_image_orientationmode(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.orientationmode",
        "params": {
            "property": "image.orientationmode",
            "value": value
        }
    });

    session.call(payload).await
}
///Bottom edge.
pub async fn set_image_processing_blacklevel_basicblacklevel_bottom(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.blacklevel.basicblacklevel.bottom",
        "params": {
            "property": "image.processing.blacklevel.basicblacklevel.bottom",
            "value": value
        }
    });

    session.call(payload).await
}
///Description not provided
pub async fn set_image_processing_blacklevel_basicblacklevel_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.blacklevel.basicblacklevel.enable",
        "params": {
            "property": "image.processing.blacklevel.basicblacklevel.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///Left edge.
pub async fn set_image_processing_blacklevel_basicblacklevel_left(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.blacklevel.basicblacklevel.left",
        "params": {
            "property": "image.processing.blacklevel.basicblacklevel.left",
            "value": value
        }
    });

    session.call(payload).await
}
///Change the upper level of the black level adjustment
pub async fn set_image_processing_blacklevel_basicblacklevel_level(
    session: &mut PulseSession,
    value: u64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.blacklevel.basicblacklevel.level",
        "params": {
            "property": "image.processing.blacklevel.basicblacklevel.level",
            "value": value
        }
    });

    session.call(payload).await
}
///Right edge.
pub async fn set_image_processing_blacklevel_basicblacklevel_right(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.blacklevel.basicblacklevel.right",
        "params": {
            "property": "image.processing.blacklevel.basicblacklevel.right",
            "value": value
        }
    });

    session.call(payload).await
}
///Top edge.
pub async fn set_image_processing_blacklevel_basicblacklevel_top(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.blacklevel.basicblacklevel.top",
        "params": {
            "property": "image.processing.blacklevel.basicblacklevel.top",
            "value": value
        }
    });

    session.call(payload).await
}
///The gain blue for black level
pub async fn set_image_processing_blacklevel_bluegain(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.blacklevel.bluegain",
        "params": {
            "property": "image.processing.blacklevel.bluegain",
            "value": value
        }
    });

    session.call(payload).await
}
///Enable/Disable black level correction
pub async fn set_image_processing_blacklevel_file_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.blacklevel.file.enable",
        "params": {
            "property": "image.processing.blacklevel.file.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///Currently selected file
pub async fn set_image_processing_blacklevel_file_selected(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.blacklevel.file.selected",
        "params": {
            "property": "image.processing.blacklevel.file.selected",
            "value": value
        }
    });

    session.call(payload).await
}
///The gain green for black level
pub async fn set_image_processing_blacklevel_greengain(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.blacklevel.greengain",
        "params": {
            "property": "image.processing.blacklevel.greengain",
            "value": value
        }
    });

    session.call(payload).await
}
///When enabled and setting one color, the others will move in the same direction
pub async fn set_image_processing_blacklevel_linkrgb(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.blacklevel.linkrgb",
        "params": {
            "property": "image.processing.blacklevel.linkrgb",
            "value": value
        }
    });

    session.call(payload).await
}
///The gain red for black level
pub async fn set_image_processing_blacklevel_redgain(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.blacklevel.redgain",
        "params": {
            "property": "image.processing.blacklevel.redgain",
            "value": value
        }
    });

    session.call(payload).await
}
///Bottom blend edge.
pub async fn set_image_processing_blend_basicblend_bottom(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.blend.basicblend.bottom",
        "params": {
            "property": "image.processing.blend.basicblend.bottom",
            "value": value
        }
    });

    session.call(payload).await
}
///Description not provided
pub async fn set_image_processing_blend_basicblend_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.blend.basicblend.enable",
        "params": {
            "property": "image.processing.blend.basicblend.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///Left blend edge.
pub async fn set_image_processing_blend_basicblend_left(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.blend.basicblend.left",
        "params": {
            "property": "image.processing.blend.basicblend.left",
            "value": value
        }
    });

    session.call(payload).await
}
///Right blend edge.
pub async fn set_image_processing_blend_basicblend_right(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.blend.basicblend.right",
        "params": {
            "property": "image.processing.blend.basicblend.right",
            "value": value
        }
    });

    session.call(payload).await
}
///Top blend edge.
pub async fn set_image_processing_blend_basicblend_top(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.blend.basicblend.top",
        "params": {
            "property": "image.processing.blend.basicblend.top",
            "value": value
        }
    });

    session.call(payload).await
}
///Enable/Disable file blend
pub async fn set_image_processing_blend_file_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.blend.file.enable",
        "params": {
            "property": "image.processing.blend.file.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///Max number of selected files
pub async fn get_image_processing_blend_file_maxselected(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.processing.blend.file.maxselected","params":{"property":"image.processing.blend.file.maxselected"}});

    session.call(payload).await
}
///Currently selected files
pub async fn set_image_processing_blend_file_selected(
    session: &mut PulseSession,
    value: std::vec::Vec<String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.blend.file.selected",
        "params": {
            "property": "image.processing.blend.file.selected",
            "value": value
        }
    });

    session.call(payload).await
}
///S-Curve exponent strength.
pub async fn set_image_processing_blend_scurve(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.blend.scurve",
        "params": {
            "property": "image.processing.blend.scurve",
            "value": value
        }
    });

    session.call(payload).await
}
///Enable/Disable tilted brightness
pub async fn set_image_processing_tiltedbrightness_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.tiltedbrightness.enable",
        "params": {
            "property": "image.processing.tiltedbrightness.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///horizontal brightness correction
pub async fn set_image_processing_tiltedbrightness_horizontalcorrection(
    session: &mut PulseSession,
    value: u64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.tiltedbrightness.horizontalcorrection",
        "params": {
            "property": "image.processing.tiltedbrightness.horizontalcorrection",
            "value": value
        }
    });

    session.call(payload).await
}
///vertical brightness correction
pub async fn set_image_processing_tiltedbrightness_verticalcorrection(
    session: &mut PulseSession,
    value: u64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.tiltedbrightness.verticalcorrection",
        "params": {
            "property": "image.processing.tiltedbrightness.verticalcorrection",
            "value": value
        }
    });

    session.call(payload).await
}
///Actual transport delay.
pub async fn get_image_processing_transportdelay_actual(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.processing.transportdelay.actual","params":{"property":"image.processing.transportdelay.actual"}});

    session.call(payload).await
}
///Desired transport delay.
pub async fn set_image_processing_transportdelay_desired(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.transportdelay.desired",
        "params": {
            "property": "image.processing.transportdelay.desired",
            "value": value
        }
    });

    session.call(payload).await
}
///Minimum transport delay.
pub async fn get_image_processing_transportdelay_minimum(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.processing.transportdelay.minimum","params":{"property":"image.processing.transportdelay.minimum"}});

    session.call(payload).await
}
///U vector for bottom left corner. Positive angle is outwards.
pub async fn set_image_processing_warp_bow_bottomleftu(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.bow.bottomleftu",
        "params": {
            "property": "image.processing.warp.bow.bottomleftu",
            "value": value
        }
    });

    session.call(payload).await
}
///V vector for bottom left corner. Positive angle is outwards.
pub async fn set_image_processing_warp_bow_bottomleftv(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.bow.bottomleftv",
        "params": {
            "property": "image.processing.warp.bow.bottomleftv",
            "value": value
        }
    });

    session.call(payload).await
}
///U vector for bottom right corner. Positive angle is outwards.
pub async fn set_image_processing_warp_bow_bottomrightu(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.bow.bottomrightu",
        "params": {
            "property": "image.processing.warp.bow.bottomrightu",
            "value": value
        }
    });

    session.call(payload).await
}
///V vector for bottom right corner. Positive angle is outwards.
pub async fn set_image_processing_warp_bow_bottomrightv(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.bow.bottomrightv",
        "params": {
            "property": "image.processing.warp.bow.bottomrightv",
            "value": value
        }
    });

    session.call(payload).await
}
///Enable/Disable bow warp
pub async fn set_image_processing_warp_bow_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.bow.enable",
        "params": {
            "property": "image.processing.warp.bow.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///Enable/Disable symmetric mode.
pub async fn set_image_processing_warp_bow_symmetric(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.bow.symmetric",
        "params": {
            "property": "image.processing.warp.bow.symmetric",
            "value": value
        }
    });

    session.call(payload).await
}
///U vector for top left corner. Positive angle is outwards.
pub async fn set_image_processing_warp_bow_topleftu(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.bow.topleftu",
        "params": {
            "property": "image.processing.warp.bow.topleftu",
            "value": value
        }
    });

    session.call(payload).await
}
///V vector for top left corner. Positive angle is outwards.
pub async fn set_image_processing_warp_bow_topleftv(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.bow.topleftv",
        "params": {
            "property": "image.processing.warp.bow.topleftv",
            "value": value
        }
    });

    session.call(payload).await
}
///U vector for top right corner. Positive angle is outwards.
pub async fn set_image_processing_warp_bow_toprightu(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.bow.toprightu",
        "params": {
            "property": "image.processing.warp.bow.toprightu",
            "value": value
        }
    });

    session.call(payload).await
}
///V vector for top right corner. Positive angle is outwards.
pub async fn set_image_processing_warp_bow_toprightv(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.bow.toprightv",
        "params": {
            "property": "image.processing.warp.bow.toprightv",
            "value": value
        }
    });

    session.call(payload).await
}
///Enable/Disable all warp functions
pub async fn set_image_processing_warp_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.enable",
        "params": {
            "property": "image.processing.warp.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///Enable/Disable file warp
pub async fn set_image_processing_warp_file_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.file.enable",
        "params": {
            "property": "image.processing.warp.file.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///Currently selected file
pub async fn set_image_processing_warp_file_selected(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.file.selected",
        "params": {
            "property": "image.processing.warp.file.selected",
            "value": value
        }
    });

    session.call(payload).await
}
///Bottom left coordinate in output resolution. Negative values alowed to go outside displayed area.
pub async fn set_image_processing_warp_fourcorners_bottomleft(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.fourcorners.bottomleft",
        "params": {
            "property": "image.processing.warp.fourcorners.bottomleft",
            "value": value
        }
    });

    session.call(payload).await
}
///Bottom right coordinate in output resolution. Negative values alowed to go outside displayed area.
pub async fn set_image_processing_warp_fourcorners_bottomright(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.fourcorners.bottomright",
        "params": {
            "property": "image.processing.warp.fourcorners.bottomright",
            "value": value
        }
    });

    session.call(payload).await
}
///Enable/Disable FourCorners warp
pub async fn set_image_processing_warp_fourcorners_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.fourcorners.enable",
        "params": {
            "property": "image.processing.warp.fourcorners.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///[DEPRECATED] The height of the screen we are projecting on. Only used as in the ratio ScreenWidth/ScreenHeight, hence the unit is arbitrary.
pub async fn set_image_processing_warp_fourcorners_screenheight(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.fourcorners.screenheight",
        "params": {
            "property": "image.processing.warp.fourcorners.screenheight",
            "value": value
        }
    });

    session.call(payload).await
}
///[DEPRECATED] The width of the screen we are projecting on. Only used as in the ratio ScreenWidth/ScreenHeight, hence the unit is arbitrary.
pub async fn set_image_processing_warp_fourcorners_screenwidth(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.fourcorners.screenwidth",
        "params": {
            "property": "image.processing.warp.fourcorners.screenwidth",
            "value": value
        }
    });

    session.call(payload).await
}
///Top left coordinate in output resolution. Negative values alowed to go outside displayed area.
pub async fn set_image_processing_warp_fourcorners_topleft(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.fourcorners.topleft",
        "params": {
            "property": "image.processing.warp.fourcorners.topleft",
            "value": value
        }
    });

    session.call(payload).await
}
///Top right coordinate in output resolution. Negative values alowed to go outside displayed area.
pub async fn set_image_processing_warp_fourcorners_topright(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.fourcorners.topright",
        "params": {
            "property": "image.processing.warp.fourcorners.topright",
            "value": value
        }
    });

    session.call(payload).await
}
///Enable/Disable Position warp
pub async fn set_image_processing_warp_position_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.position.enable",
        "params": {
            "property": "image.processing.warp.position.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///The horizontal displacement factor relative to output resolution. From -HRes/2 to + HRes/2
pub async fn set_image_processing_warp_position_horizontal(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.position.horizontal",
        "params": {
            "property": "image.processing.warp.position.horizontal",
            "value": value
        }
    });

    session.call(payload).await
}
///The vertical displacement factor relative to output resolution. From -VRes/2 to +VRes/2
pub async fn set_image_processing_warp_position_vertical(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.position.vertical",
        "params": {
            "property": "image.processing.warp.position.vertical",
            "value": value
        }
    });

    session.call(payload).await
}
///The warp status.
pub async fn get_image_processing_warp_warpstatus(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.processing.warp.warpstatus","params":{"property":"image.processing.warp.warpstatus"}});

    session.call(payload).await
}
///Warp status description
pub async fn get_image_processing_warp_warpstatusdescription(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.processing.warp.warpstatusdescription","params":{"property":"image.processing.warp.warpstatusdescription"}});

    session.call(payload).await
}
///Enable/Disable Zoom warp
pub async fn set_image_processing_warp_zoom_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.zoom.enable",
        "params": {
            "property": "image.processing.warp.zoom.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///The zoom factor
pub async fn set_image_processing_warp_zoom_factor(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.processing.warp.zoom.factor",
        "params": {
            "property": "image.processing.warp.zoom.factor",
            "value": value
        }
    });

    session.call(payload).await
}
///Resulting zoomed resolution
pub async fn get_image_processing_warp_zoom_size(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.processing.warp.zoom.size","params":{"property":"image.processing.warp.zoom.size"}});

    session.call(payload).await
}
///The current resolution size (pixels x lines).
pub async fn get_image_resolution_alpha_size(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.resolution.alpha.size","params":{"property":"image.resolution.alpha.size"}});

    session.call(payload).await
}
///List all resolutions that can currently be selected.
pub async fn get_image_resolution_availableresolutions(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.resolution.availableresolutions","params":{"property":"image.resolution.availableresolutions"}});

    session.call(payload).await
}
///The current resolution size (pixels x lines).
pub async fn get_image_resolution_beta_size(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.resolution.beta.size","params":{"property":"image.resolution.beta.size"}});

    session.call(payload).await
}
///The current resolution size (pixels x lines).
pub async fn get_image_resolution_display_size(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.resolution.display.size","params":{"property":"image.resolution.display.size"}});

    session.call(payload).await
}
///The current resolution size (pixels x lines).
pub async fn get_image_resolution_osd_size(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.resolution.osd.size","params":{"property":"image.resolution.osd.size"}});

    session.call(payload).await
}
///The current resolution size (pixels x lines).
pub async fn get_image_resolution_output_size(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.resolution.output.size","params":{"property":"image.resolution.output.size"}});

    session.call(payload).await
}
///The current resolution size (pixels x lines).
pub async fn get_image_resolution_processing_size(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.resolution.processing.size","params":{"property":"image.resolution.processing.size"}});

    session.call(payload).await
}
///The current resolution description.
pub async fn set_image_resolution_resolution(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.resolution.resolution",
        "params": {
            "property": "image.resolution.resolution",
            "value": value
        }
    });

    session.call(payload).await
}
///The current resolution size (pixels x lines).
pub async fn get_image_resolution_size(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.resolution.size","params":{"property":"image.resolution.size"}});

    session.call(payload).await
}
///Image color saturation. The value is normalized, 1 is default.
pub async fn set_image_saturation(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.saturation",
        "params": {
            "property": "image.saturation",
            "value": value
        }
    });

    session.call(payload).await
}
///Image sharpness. The value is normalized.
pub async fn set_image_sharpness(
    session: &mut PulseSession,
    value: u64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.sharpness",
        "params": {
            "property": "image.sharpness",
            "value": value
        }
    });

    session.call(payload).await
}
///Capabilities
pub async fn get_image_source_displayport_capabilities(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.source.displayport.capabilities","params":{"property":"image.source.displayport.capabilities"}});

    session.call(payload).await
}
///Source layout
pub async fn get_image_source_displayport_layout(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.source.displayport.layout","params":{"property":"image.source.displayport.layout"}});

    session.call(payload).await
}
///Name of the source
pub async fn get_image_source_displayport_name(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.source.displayport.name","params":{"property":"image.source.displayport.name"}});

    session.call(payload).await
}
///Capabilities
pub async fn get_image_source_hdmi_capabilities(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.source.hdmi.capabilities","params":{"property":"image.source.hdmi.capabilities"}});

    session.call(payload).await
}
///Source layout
pub async fn get_image_source_hdmi_layout(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.source.hdmi.layout","params":{"property":"image.source.hdmi.layout"}});

    session.call(payload).await
}
///Name of the source
pub async fn get_image_source_hdmi_name(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.source.hdmi.name","params":{"property":"image.source.hdmi.name"}});

    session.call(payload).await
}
///Darktime in us.
pub async fn set_image_stereo_darktime(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.stereo.darktime",
        "params": {
            "property": "image.stereo.darktime",
            "value": value
        }
    });

    session.call(payload).await
}
///Sync delay in us.
pub async fn set_image_stereo_glassync_delay(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.stereo.glassync.delay",
        "params": {
            "property": "image.stereo.glassync.delay",
            "value": value
        }
    });

    session.call(payload).await
}
///Maximum sync delay in us.
pub async fn get_image_stereo_glassync_delaymaximum(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.stereo.glassync.delaymaximum","params":{"property":"image.stereo.glassync.delaymaximum"}});

    session.call(payload).await
}
///Minimum sync delay in us.
pub async fn get_image_stereo_glassync_delayminimum(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.stereo.glassync.delayminimum","params":{"property":"image.stereo.glassync.delayminimum"}});

    session.call(payload).await
}
///Sync invert.
pub async fn set_image_stereo_glassync_invert(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.stereo.glassync.invert",
        "params": {
            "property": "image.stereo.glassync.invert",
            "value": value
        }
    });

    session.call(payload).await
}
///swap which stereo frames belong to each other
pub async fn set_image_stereo_swapframepair(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.stereo.swapframepair",
        "params": {
            "property": "image.stereo.swapframepair",
            "value": value
        }
    });

    session.call(payload).await
}
///When false, the service will deny outside property change requests
pub async fn get_image_testpattern_available(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.testpattern.available","params":{"property":"image.testpattern.available"}});

    session.call(payload).await
}
///Enables or disables the ability to upload download and delete custom testpatterns
pub async fn get_image_testpattern_enablefilemanagement(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.testpattern.enablefilemanagement","params":{"property":"image.testpattern.enablefilemanagement"}});

    session.call(payload).await
}
///The list of testpatterns to iterate in the viewing order.
pub async fn get_image_testpattern_iterator_calibration_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.testpattern.iterator.calibration.list","params":{"property":"image.testpattern.iterator.calibration.list"}});

    session.call(payload).await
}
///Return the size of the list of test patterns to iterate upon.
pub async fn get_image_testpattern_iterator_calibration_size(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.testpattern.iterator.calibration.size","params":{"property":"image.testpattern.iterator.calibration.size"}});

    session.call(payload).await
}
///The list of testpatterns to iterate in the viewing order.
pub async fn get_image_testpattern_iterator_iris_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.testpattern.iterator.iris.list","params":{"property":"image.testpattern.iterator.iris.list"}});

    session.call(payload).await
}
///Return the size of the list of test patterns to iterate upon.
pub async fn get_image_testpattern_iterator_iris_size(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.testpattern.iterator.iris.size","params":{"property":"image.testpattern.iterator.iris.size"}});

    session.call(payload).await
}
///The list of testpatterns to iterate in the viewing order.
pub async fn get_image_testpattern_iterator_optics_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.testpattern.iterator.optics.list","params":{"property":"image.testpattern.iterator.optics.list"}});

    session.call(payload).await
}
///Return the size of the list of test patterns to iterate upon.
pub async fn get_image_testpattern_iterator_optics_size(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.testpattern.iterator.optics.size","params":{"property":"image.testpattern.iterator.optics.size"}});

    session.call(payload).await
}
///The list of testpatterns to iterate in the viewing order.
pub async fn get_image_testpattern_iterator_shift_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.testpattern.iterator.shift.list","params":{"property":"image.testpattern.iterator.shift.list"}});

    session.call(payload).await
}
///Return the size of the list of test patterns to iterate upon.
pub async fn get_image_testpattern_iterator_shift_size(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.testpattern.iterator.shift.size","params":{"property":"image.testpattern.iterator.shift.size"}});

    session.call(payload).await
}
///The list of testpatterns to iterate in the viewing order.
pub async fn get_image_testpattern_iterator_stereo_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.testpattern.iterator.stereo.list","params":{"property":"image.testpattern.iterator.stereo.list"}});

    session.call(payload).await
}
///Return the size of the list of test patterns to iterate upon.
pub async fn get_image_testpattern_iterator_stereo_size(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.testpattern.iterator.stereo.size","params":{"property":"image.testpattern.iterator.stereo.size"}});

    session.call(payload).await
}
///The list of testpatterns to iterate in the viewing order.
pub async fn get_image_testpattern_iterator_xpr_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.testpattern.iterator.xpr.list","params":{"property":"image.testpattern.iterator.xpr.list"}});

    session.call(payload).await
}
///Return the size of the list of test patterns to iterate upon.
pub async fn get_image_testpattern_iterator_xpr_size(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.testpattern.iterator.xpr.size","params":{"property":"image.testpattern.iterator.xpr.size"}});

    session.call(payload).await
}
///The unique test pattern id to show as the NoSignal pattern
pub async fn set_image_testpattern_nosignal_pattern(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.testpattern.nosignal.pattern",
        "params": {
            "property": "image.testpattern.nosignal.pattern",
            "value": value
        }
    });

    session.call(payload).await
}
///The unique id of the selected pattern
pub async fn set_image_testpattern_selected(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.testpattern.selected",
        "params": {
            "property": "image.testpattern.selected",
            "value": value
        }
    });

    session.call(payload).await
}
///Enable/disable the display of the selected pattern
pub async fn set_image_testpattern_show(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.testpattern.show",
        "params": {
            "property": "image.testpattern.show",
            "value": value
        }
    });

    session.call(payload).await
}
///Indicates whether or not the currently selected source has an active signal (true) or not (false)
pub async fn get_image_window_main_activesignal(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.window.main.activesignal","params":{"property":"image.window.main.activesignal"}});

    session.call(payload).await
}
///Window position
pub async fn get_image_window_main_position(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.window.main.position","params":{"property":"image.window.main.position"}});

    session.call(payload).await
}
///The scaling mode to apply to the source
pub async fn set_image_window_main_scalingmode(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.window.main.scalingmode",
        "params": {
            "property": "image.window.main.scalingmode",
            "value": value
        }
    });

    session.call(payload).await
}
///Window size
pub async fn get_image_window_main_size(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.window.main.size","params":{"property":"image.window.main.size"}});

    session.call(payload).await
}
///The source that is displayed in this window
pub async fn set_image_window_main_source(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "image.window.main.source",
        "params": {
            "property": "image.window.main.source",
            "value": value
        }
    });

    session.call(payload).await
}
///The reason(s) the source is not valid
pub async fn get_image_window_main_sourceinvalidityreasons(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"image.window.main.sourceinvalidityreasons","params":{"property":"image.window.main.sourceinvalidityreasons"}});

    session.call(payload).await
}
///True if the certificate applies to this projector
pub async fn get_iot_certificate_applicable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"iot.certificate.applicable","params":{"property":"iot.certificate.applicable"}});

    session.call(payload).await
}
///True if a certificate was found
pub async fn get_iot_certificate_available(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"iot.certificate.available","params":{"property":"iot.certificate.available"}});

    session.call(payload).await
}
///certificate Issuer info
pub async fn get_iot_certificate_issuer(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"iot.certificate.issuer","params":{"property":"iot.certificate.issuer"}});

    session.call(payload).await
}
///Result of the validity check of the certificate
pub async fn get_iot_certificate_valid(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"iot.certificate.valid","params":{"property":"iot.certificate.valid"}});

    session.call(payload).await
}
///certificate Valid until this date
pub async fn get_iot_certificate_validityend(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"iot.certificate.validityend","params":{"property":"iot.certificate.validityend"}});

    session.call(payload).await
}
///certificate Valid from this date
pub async fn get_iot_certificate_validitystart(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"iot.certificate.validitystart","params":{"property":"iot.certificate.validitystart"}});

    session.call(payload).await
}
///The current state of the connection.
pub async fn get_iot_cloudservices_connectionstate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"iot.cloudservices.connectionstate","params":{"property":"iot.cloudservices.connectionstate"}});

    session.call(payload).await
}
///True if IOT cloud connection is enabled, false otherwise
pub async fn set_iot_cloudservices_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "iot.cloudservices.enable",
        "params": {
            "property": "iot.cloudservices.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///This is the address (hostname/ip) of the proxy.
pub async fn set_iot_proxy_address(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "iot.proxy.address",
        "params": {
            "property": "iot.proxy.address",
            "value": value
        }
    });

    session.call(payload).await
}
///When enabled, iot connection will go over proxy specified in othe proxy parameters
pub async fn set_iot_proxy_enabled(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "iot.proxy.enabled",
        "params": {
            "property": "iot.proxy.enabled",
            "value": value
        }
    });

    session.call(payload).await
}
///Password used to log in to proxy.
pub async fn set_iot_proxy_password(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "iot.proxy.password",
        "params": {
            "property": "iot.proxy.password",
            "value": value
        }
    });

    session.call(payload).await
}
///The port to use for the proxy connection.
pub async fn set_iot_proxy_port(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "iot.proxy.port",
        "params": {
            "property": "iot.proxy.port",
            "value": value
        }
    });

    session.call(payload).await
}
///Username used to log in to proxy.
pub async fn set_iot_proxy_username(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "iot.proxy.username",
        "params": {
            "property": "iot.proxy.username",
            "value": value
        }
    });

    session.call(payload).await
}
///Indiates whether or not the user is required to accept the term agreement.
pub async fn get_iot_requiretermagreement(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"iot.requiretermagreement","params":{"property":"iot.requiretermagreement"}});

    session.call(payload).await
}
///Represents the users acknowledge of the IOT terms of agreement.
pub async fn set_iot_termagreement(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "iot.termagreement",
        "params": {
            "property": "iot.termagreement",
            "value": value
        }
    });

    session.call(payload).await
}
///Initial key repeat delay in milliseconds
pub async fn get_keydispatcher_repeatdelay(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"keydispatcher.repeatdelay","params":{"property":"keydispatcher.repeatdelay"}});

    session.call(payload).await
}
///Key repeat interval in milliseconds
pub async fn get_keydispatcher_repeatinterval(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"keydispatcher.repeatinterval","params":{"property":"keydispatcher.repeatinterval"}});

    session.call(payload).await
}
///Whether the device has carrier or not
pub async fn get_network_device_lan_carrier(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"network.device.lan.carrier","params":{"property":"network.device.lan.carrier"}});

    session.call(payload).await
}
///The configuration method of the device: auto or manual
pub async fn set_network_device_lan_configuration(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "network.device.lan.configuration",
        "params": {
            "property": "network.device.lan.configuration",
            "value": value
        }
    });

    session.call(payload).await
}
///The general type of the network device
pub async fn get_network_device_lan_devicetype(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"network.device.lan.devicetype","params":{"property":"network.device.lan.devicetype"}});

    session.call(payload).await
}
///The active hardware (MAC) address
pub async fn get_network_device_lan_hwaddress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"network.device.lan.hwaddress","params":{"property":"network.device.lan.hwaddress"}});

    session.call(payload).await
}
///The current configuration for IP version 4
pub async fn get_network_device_lan_ip4config(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"network.device.lan.ip4config","params":{"property":"network.device.lan.ip4config"}});

    session.call(payload).await
}
///Get/set the manual configuration for IP version 4
pub async fn set_network_device_lan_ip4configmanual(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "network.device.lan.ip4configmanual",
        "params": {
            "property": "network.device.lan.ip4configmanual",
            "value": value
        }
    });

    session.call(payload).await
}
///The current configuration for IP version 6
pub async fn get_network_device_lan_ip6config(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"network.device.lan.ip6config","params":{"property":"network.device.lan.ip6config"}});

    session.call(payload).await
}
///Get/set the manual configuration for IP version 4
pub async fn set_network_device_lan_ip6configmanual(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "network.device.lan.ip6configmanual",
        "params": {
            "property": "network.device.lan.ip6configmanual",
            "value": value
        }
    });

    session.call(payload).await
}
///The speed of the device in Mbit/s
pub async fn get_network_device_lan_speed(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"network.device.lan.speed","params":{"property":"network.device.lan.speed"}});

    session.call(payload).await
}
///The current state of the device
pub async fn get_network_device_lan_state(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"network.device.lan.state","params":{"property":"network.device.lan.state"}});

    session.call(payload).await
}
///Additional information about the device state. Can be empty
pub async fn get_network_device_lan_stateinfo(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"network.device.lan.stateinfo","params":{"property":"network.device.lan.stateinfo"}});

    session.call(payload).await
}
///The domain name
pub async fn set_network_domain(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "network.domain",
        "params": {
            "property": "network.domain",
            "value": value
        }
    });

    session.call(payload).await
}
///The host name
pub async fn set_network_hostname(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "network.hostname",
        "params": {
            "property": "network.hostname",
            "value": value
        }
    });

    session.call(payload).await
}
///The Networking Service version
pub async fn get_network_version(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"network.version","params":{"property":"network.version"}});

    session.call(payload).await
}
///The number of notifications received and dismissed
pub async fn get_notification_count(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"notification.count","params":{"property":"notification.count"}});

    session.call(payload).await
}
///Current calibration state
pub async fn get_optics_focus_calibrationstate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.focus.calibrationstate","params":{"property":"optics.focus.calibrationstate"}});

    session.call(payload).await
}
///Enabled state
pub async fn set_optics_focus_enabled(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "optics.focus.enabled",
        "params": {
            "property": "optics.focus.enabled",
            "value": value
        }
    });

    session.call(payload).await
}
///Forward limit reached
pub async fn get_optics_focus_limits_forward(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.focus.limits.forward","params":{"property":"optics.focus.limits.forward"}});

    session.call(payload).await
}
///Reverse limit reached
pub async fn get_optics_focus_limits_reverse(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.focus.limits.reverse","params":{"property":"optics.focus.limits.reverse"}});

    session.call(payload).await
}
///Maximum available position
pub async fn get_optics_focus_maxposition(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.focus.maxposition","params":{"property":"optics.focus.maxposition"}});

    session.call(payload).await
}
///Minimum available position
pub async fn get_optics_focus_minposition(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.focus.minposition","params":{"property":"optics.focus.minposition"}});

    session.call(payload).await
}
///Current position
pub async fn get_optics_focus_position(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.focus.position","params":{"property":"optics.focus.position"}});

    session.call(payload).await
}
///Safe to calibrate
pub async fn get_optics_focus_safetocalibrate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.focus.safetocalibrate","params":{"property":"optics.focus.safetocalibrate"}});

    session.call(payload).await
}
///Safe to operate state
pub async fn get_optics_focus_safetooperate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.focus.safetooperate","params":{"property":"optics.focus.safetooperate"}});

    session.call(payload).await
}
///Current state
pub async fn get_optics_focus_state(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.focus.state","params":{"property":"optics.focus.state"}});

    session.call(payload).await
}
///Desired target
pub async fn set_optics_focus_target(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "optics.focus.target",
        "params": {
            "property": "optics.focus.target",
            "value": value
        }
    });

    session.call(payload).await
}
///Property for selecting whether iris operation should be linked or individual
pub async fn set_optics_irisoperation(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "optics.irisoperation",
        "params": {
            "property": "optics.irisoperation",
            "value": value
        }
    });

    session.call(payload).await
}
///Description not provided
pub async fn get_optics_irisoperationavailable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.irisoperationavailable","params":{"property":"optics.irisoperationavailable"}});

    session.call(payload).await
}
///Currently selected lens
pub async fn get_optics_lens_lens(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lens.lens","params":{"property":"optics.lens.lens"}});

    session.call(payload).await
}
///Current lens metadata
pub async fn get_optics_lens_metadata(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lens.metadata","params":{"property":"optics.lens.metadata"}});

    session.call(payload).await
}
///List of available lens positions
pub async fn get_optics_lens_position_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lens.position.list","params":{"property":"optics.lens.position.list"}});

    session.call(payload).await
}
///Currently selected lens position
pub async fn set_optics_lens_position_selected(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "optics.lens.position.selected",
        "params": {
            "property": "optics.lens.position.selected",
            "value": value
        }
    });

    session.call(payload).await
}
///Lens positions stored focus position
pub async fn get_optics_lens_position_test1_focus(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lens.position.test1.focus","params":{"property":"optics.lens.position.test1.focus"}});

    session.call(payload).await
}
///Lens positions name
pub async fn get_optics_lens_position_test1_name(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lens.position.test1.name","params":{"property":"optics.lens.position.test1.name"}});

    session.call(payload).await
}
///Lens positions stored horizontal shift position
pub async fn get_optics_lens_position_test1_shifthorizontal(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lens.position.test1.shifthorizontal","params":{"property":"optics.lens.position.test1.shifthorizontal"}});

    session.call(payload).await
}
///Lens positions stored vertical shift position
pub async fn get_optics_lens_position_test1_shiftvertical(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lens.position.test1.shiftvertical","params":{"property":"optics.lens.position.test1.shiftvertical"}});

    session.call(payload).await
}
///Lens position status
pub async fn get_optics_lens_position_test1_status(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lens.position.test1.status","params":{"property":"optics.lens.position.test1.status"}});

    session.call(payload).await
}
///Lens positions stored zoom position
pub async fn get_optics_lens_position_test1_zoom(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lens.position.test1.zoom","params":{"property":"optics.lens.position.test1.zoom"}});

    session.call(payload).await
}
///Throw ratios of the selected lens
pub async fn get_optics_lens_throwratio(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lens.throwratio","params":{"property":"optics.lens.throwratio"}});

    session.call(payload).await
}
///Lens present
pub async fn get_optics_lenspresent(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lenspresent","params":{"property":"optics.lenspresent"}});

    session.call(payload).await
}
///Current calibration state
pub async fn get_optics_lensshift_horizontal_calibrationstate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lensshift.horizontal.calibrationstate","params":{"property":"optics.lensshift.horizontal.calibrationstate"}});

    session.call(payload).await
}
///Enabled state
pub async fn set_optics_lensshift_horizontal_enabled(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "optics.lensshift.horizontal.enabled",
        "params": {
            "property": "optics.lensshift.horizontal.enabled",
            "value": value
        }
    });

    session.call(payload).await
}
///Forward limit reached
pub async fn get_optics_lensshift_horizontal_limits_forward(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lensshift.horizontal.limits.forward","params":{"property":"optics.lensshift.horizontal.limits.forward"}});

    session.call(payload).await
}
///Reverse limit reached
pub async fn get_optics_lensshift_horizontal_limits_reverse(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lensshift.horizontal.limits.reverse","params":{"property":"optics.lensshift.horizontal.limits.reverse"}});

    session.call(payload).await
}
///Maximum available position
pub async fn get_optics_lensshift_horizontal_maxposition(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lensshift.horizontal.maxposition","params":{"property":"optics.lensshift.horizontal.maxposition"}});

    session.call(payload).await
}
///Minimum available position
pub async fn get_optics_lensshift_horizontal_minposition(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lensshift.horizontal.minposition","params":{"property":"optics.lensshift.horizontal.minposition"}});

    session.call(payload).await
}
///Current position
pub async fn get_optics_lensshift_horizontal_position(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lensshift.horizontal.position","params":{"property":"optics.lensshift.horizontal.position"}});

    session.call(payload).await
}
///Safe to calibrate
pub async fn get_optics_lensshift_horizontal_safetocalibrate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lensshift.horizontal.safetocalibrate","params":{"property":"optics.lensshift.horizontal.safetocalibrate"}});

    session.call(payload).await
}
///Safe to operate state
pub async fn get_optics_lensshift_horizontal_safetooperate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lensshift.horizontal.safetooperate","params":{"property":"optics.lensshift.horizontal.safetooperate"}});

    session.call(payload).await
}
///Current state
pub async fn get_optics_lensshift_horizontal_state(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lensshift.horizontal.state","params":{"property":"optics.lensshift.horizontal.state"}});

    session.call(payload).await
}
///Desired target
pub async fn set_optics_lensshift_horizontal_target(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "optics.lensshift.horizontal.target",
        "params": {
            "property": "optics.lensshift.horizontal.target",
            "value": value
        }
    });

    session.call(payload).await
}
///Current calibration state
pub async fn get_optics_lensshift_vertical_calibrationstate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lensshift.vertical.calibrationstate","params":{"property":"optics.lensshift.vertical.calibrationstate"}});

    session.call(payload).await
}
///Enabled state
pub async fn set_optics_lensshift_vertical_enabled(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "optics.lensshift.vertical.enabled",
        "params": {
            "property": "optics.lensshift.vertical.enabled",
            "value": value
        }
    });

    session.call(payload).await
}
///Forward limit reached
pub async fn get_optics_lensshift_vertical_limits_forward(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lensshift.vertical.limits.forward","params":{"property":"optics.lensshift.vertical.limits.forward"}});

    session.call(payload).await
}
///Reverse limit reached
pub async fn get_optics_lensshift_vertical_limits_reverse(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lensshift.vertical.limits.reverse","params":{"property":"optics.lensshift.vertical.limits.reverse"}});

    session.call(payload).await
}
///Maximum available position
pub async fn get_optics_lensshift_vertical_maxposition(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lensshift.vertical.maxposition","params":{"property":"optics.lensshift.vertical.maxposition"}});

    session.call(payload).await
}
///Minimum available position
pub async fn get_optics_lensshift_vertical_minposition(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lensshift.vertical.minposition","params":{"property":"optics.lensshift.vertical.minposition"}});

    session.call(payload).await
}
///Current position
pub async fn get_optics_lensshift_vertical_position(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lensshift.vertical.position","params":{"property":"optics.lensshift.vertical.position"}});

    session.call(payload).await
}
///Safe to calibrate
pub async fn get_optics_lensshift_vertical_safetocalibrate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lensshift.vertical.safetocalibrate","params":{"property":"optics.lensshift.vertical.safetocalibrate"}});

    session.call(payload).await
}
///Safe to operate state
pub async fn get_optics_lensshift_vertical_safetooperate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lensshift.vertical.safetooperate","params":{"property":"optics.lensshift.vertical.safetooperate"}});

    session.call(payload).await
}
///Current state
pub async fn get_optics_lensshift_vertical_state(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.lensshift.vertical.state","params":{"property":"optics.lensshift.vertical.state"}});

    session.call(payload).await
}
///Desired target
pub async fn set_optics_lensshift_vertical_target(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "optics.lensshift.vertical.target",
        "params": {
            "property": "optics.lensshift.vertical.target",
            "value": value
        }
    });

    session.call(payload).await
}
///Position of shutter
pub async fn get_optics_shutter_position(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.shutter.position","params":{"property":"optics.shutter.position"}});

    session.call(payload).await
}
///Target position of shutter
pub async fn set_optics_shutter_target(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "optics.shutter.target",
        "params": {
            "property": "optics.shutter.target",
            "value": value
        }
    });

    session.call(payload).await
}
///Current calibration state
pub async fn get_optics_zoom_calibrationstate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.zoom.calibrationstate","params":{"property":"optics.zoom.calibrationstate"}});

    session.call(payload).await
}
///Enabled state
pub async fn set_optics_zoom_enabled(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "optics.zoom.enabled",
        "params": {
            "property": "optics.zoom.enabled",
            "value": value
        }
    });

    session.call(payload).await
}
///Forward limit reached
pub async fn get_optics_zoom_limits_forward(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.zoom.limits.forward","params":{"property":"optics.zoom.limits.forward"}});

    session.call(payload).await
}
///Reverse limit reached
pub async fn get_optics_zoom_limits_reverse(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.zoom.limits.reverse","params":{"property":"optics.zoom.limits.reverse"}});

    session.call(payload).await
}
///Maximum available position
pub async fn get_optics_zoom_maxposition(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.zoom.maxposition","params":{"property":"optics.zoom.maxposition"}});

    session.call(payload).await
}
///Minimum available position
pub async fn get_optics_zoom_minposition(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.zoom.minposition","params":{"property":"optics.zoom.minposition"}});

    session.call(payload).await
}
///Current position
pub async fn get_optics_zoom_position(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.zoom.position","params":{"property":"optics.zoom.position"}});

    session.call(payload).await
}
///Safe to calibrate
pub async fn get_optics_zoom_safetocalibrate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.zoom.safetocalibrate","params":{"property":"optics.zoom.safetocalibrate"}});

    session.call(payload).await
}
///Safe to operate state
pub async fn get_optics_zoom_safetooperate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.zoom.safetooperate","params":{"property":"optics.zoom.safetooperate"}});

    session.call(payload).await
}
///Current state
pub async fn get_optics_zoom_state(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"optics.zoom.state","params":{"property":"optics.zoom.state"}});

    session.call(payload).await
}
///Desired target
pub async fn set_optics_zoom_target(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "optics.zoom.target",
        "params": {
            "property": "optics.zoom.target",
            "value": value
        }
    });

    session.call(payload).await
}
///The list of available domains for profiles.
pub async fn get_profile_domains(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"profile.domains","params":{"property":"profile.domains"}});

    session.call(payload).await
}
///All preset assigments.
pub async fn get_profile_presetassignments(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"profile.presetassignments","params":{"property":"profile.presetassignments"}});

    session.call(payload).await
}
///The list of created profiles.
pub async fn get_profile_profiles(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"profile.profiles","params":{"property":"profile.profiles"}});

    session.call(payload).await
}
///Description not provided
pub async fn set_property_notifyself(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "property.notifyself",
        "params": {
            "property": "property.notifyself",
            "value": value
        }
    });

    session.call(payload).await
}
///Enable/disable PJLink authentication procedure
pub async fn get_protocols_pjlink_authenticationrequired(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"protocols.pjlink.authenticationrequired","params":{"property":"protocols.pjlink.authenticationrequired"}});

    session.call(payload).await
}
///Enable/disable PJLink
pub async fn get_protocols_pjlink_enable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"protocols.pjlink.enable","params":{"property":"protocols.pjlink.enable"}});

    session.call(payload).await
}
///The address of the remote control that the projector will respond to
pub async fn set_remotecontrol_address(
    session: &mut PulseSession,
    value: u64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "remotecontrol.address",
        "params": {
            "property": "remotecontrol.address",
            "value": value
        }
    });

    session.call(payload).await
}
///The broadcast address
pub async fn set_remotecontrol_broadcastaddress(
    session: &mut PulseSession,
    value: u64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "remotecontrol.broadcastaddress",
        "params": {
            "property": "remotecontrol.broadcastaddress",
            "value": value
        }
    });

    session.call(payload).await
}
///Enable or disable the IR sensor
pub async fn set_remotecontrol_sensors_front_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "remotecontrol.sensors.front.enable",
        "params": {
            "property": "remotecontrol.sensors.front.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///The display name of the IR sensor
pub async fn get_remotecontrol_sensors_front_name(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"remotecontrol.sensors.front.name","params":{"property":"remotecontrol.sensors.front.name"}});

    session.call(payload).await
}
///Enable or disable the IR sensor
pub async fn set_remotecontrol_sensors_rear_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "remotecontrol.sensors.rear.enable",
        "params": {
            "property": "remotecontrol.sensors.rear.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///The display name of the IR sensor
pub async fn get_remotecontrol_sensors_rear_name(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"remotecontrol.sensors.rear.name","params":{"property":"remotecontrol.sensors.rear.name"}});

    session.call(payload).await
}
///Enable or disable the IR sensor
pub async fn set_remotecontrol_sensors_side_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "remotecontrol.sensors.side.enable",
        "params": {
            "property": "remotecontrol.sensors.side.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///The display name of the IR sensor
pub async fn get_remotecontrol_sensors_side_name(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"remotecontrol.sensors.side.name","params":{"property":"remotecontrol.sensors.side.name"}});

    session.call(payload).await
}
///Enable or disable the IR sensor
pub async fn set_remotecontrol_sensors_wiredremote_layer1_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "remotecontrol.sensors.wiredremote-layer1.enable",
        "params": {
            "property": "remotecontrol.sensors.wiredremote-layer1.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///The display name of the IR sensor
pub async fn get_remotecontrol_sensors_wiredremote_layer1_name(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"remotecontrol.sensors.wiredremote-layer1.name","params":{"property":"remotecontrol.sensors.wiredremote-layer1.name"}});

    session.call(payload).await
}
///Enable or disable the IR sensor
pub async fn set_remotecontrol_sensors_wiredremote_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "remotecontrol.sensors.wiredremote.enable",
        "params": {
            "property": "remotecontrol.sensors.wiredremote.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///The display name of the IR sensor
pub async fn get_remotecontrol_sensors_wiredremote_name(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"remotecontrol.sensors.wiredremote.name","params":{"property":"remotecontrol.sensors.wiredremote.name"}});

    session.call(payload).await
}
///List of all scheduled actions' name.
pub async fn get_scheduler_actions(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"scheduler.actions","params":{"property":"scheduler.actions"}});

    session.call(payload).await
}
///List of all available action types.
pub async fn get_scheduler_actiontypes(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"scheduler.actiontypes","params":{"property":"scheduler.actiontypes"}});

    session.call(payload).await
}
///The next task(s) that will be executed. (Can be more than one task if multiple tasks have the exact same execution time.)
pub async fn get_scheduler_nextscheduledaction(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"scheduler.nextscheduledaction","params":{"property":"scheduler.nextscheduledaction"}});

    session.call(payload).await
}
///Return True if action is paused, otherwise return False.
pub async fn set_scheduler_pause(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "scheduler.pause",
        "params": {
            "property": "scheduler.pause",
            "value": value
        }
    });

    session.call(payload).await
}
///List of all scheduled actions data.
pub async fn set_scheduler_scheduledactions(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "scheduler.scheduledactions",
        "params": {
            "property": "scheduler.scheduledactions",
            "value": value
        }
    });

    session.call(payload).await
}
///List of all available transitions for system action.
pub async fn get_scheduler_transitions(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"scheduler.transitions","params":{"property":"scheduler.transitions"}});

    session.call(payload).await
}
///[DEPRECATED] Replaced by image.processing.colormapping.hdrboost
pub async fn set_screen_hdrboost(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "screen.hdrboost",
        "params": {
            "property": "screen.hdrboost",
            "value": value
        }
    });

    session.call(payload).await
}
///The maximum luminance measured on the screen in cd/m2
pub async fn set_screen_luminance(
    session: &mut PulseSession,
    value: f64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "screen.luminance",
        "params": {
            "property": "screen.luminance",
            "value": value
        }
    });

    session.call(payload).await
}
///The calculated aspect of the ScreenWidth/ScreenHeight
pub async fn get_screen_screenaspect(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"screen.screenaspect","params":{"property":"screen.screenaspect"}});

    session.call(payload).await
}
///The width and height of the screen we are projecting on.
pub async fn set_screen_screensize(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "screen.screensize",
        "params": {
            "property": "screen.screensize",
            "value": value
        }
    });

    session.call(payload).await
}
///Counter value
pub async fn get_statistics_laser_plate01_bank01_runtimeseconds_value(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"statistics.laser.plate01.bank01.runtimeseconds.value","params":{"property":"statistics.laser.plate01.bank01.runtimeseconds.value"}});

    session.call(payload).await
}
///Counter value
pub async fn get_statistics_laser_plate01_bank02_runtimeseconds_value(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"statistics.laser.plate01.bank02.runtimeseconds.value","params":{"property":"statistics.laser.plate01.bank02.runtimeseconds.value"}});

    session.call(payload).await
}
///Counter value
pub async fn get_statistics_laser_plate02_bank01_runtimeseconds_value(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"statistics.laser.plate02.bank01.runtimeseconds.value","params":{"property":"statistics.laser.plate02.bank01.runtimeseconds.value"}});

    session.call(payload).await
}
///Counter value
pub async fn get_statistics_laser_plate02_bank02_runtimeseconds_value(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"statistics.laser.plate02.bank02.runtimeseconds.value","params":{"property":"statistics.laser.plate02.bank02.runtimeseconds.value"}});

    session.call(payload).await
}
///Counter value
pub async fn get_statistics_maintenance_actuator_value(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"statistics.maintenance.actuator.value","params":{"property":"statistics.maintenance.actuator.value"}});

    session.call(payload).await
}
///Counter value
pub async fn get_statistics_maintenance_colorwheel_value(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"statistics.maintenance.colorwheel.value","params":{"property":"statistics.maintenance.colorwheel.value"}});

    session.call(payload).await
}
///Counter value
pub async fn get_statistics_maintenance_phosphorwheel_value(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"statistics.maintenance.phosphorwheel.value","params":{"property":"statistics.maintenance.phosphorwheel.value"}});

    session.call(payload).await
}
///Counter value
pub async fn get_statistics_projector_value(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"statistics.projector.value","params":{"property":"statistics.projector.value"}});

    session.call(payload).await
}
///Article number.
pub async fn get_system_articlenumber(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.articlenumber","params":{"property":"system.articlenumber"}});

    session.call(payload).await
}
///Article number of installed color wheel
pub async fn get_system_colorwheel(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.colorwheel","params":{"property":"system.colorwheel"}});

    session.call(payload).await
}
///Name of installed color wheel
pub async fn get_system_colorwheelname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.colorwheelname","params":{"property":"system.colorwheelname"}});

    session.call(payload).await
}
///An array of all available time zones and their information.
pub async fn get_system_date_availabletimezones(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.date.availabletimezones","params":{"property":"system.date.availabletimezones"}});

    session.call(payload).await
}
///True if there is a connection to the NTP-server.
pub async fn get_system_date_ntp_connected(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.date.ntp.connected","params":{"property":"system.date.ntp.connected"}});

    session.call(payload).await
}
///True if NTP time synchronization should be used.
pub async fn get_system_date_ntp_enabled(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.date.ntp.enabled","params":{"property":"system.date.ntp.enabled"}});

    session.call(payload).await
}
///The NTP server hostname or address.
pub async fn get_system_date_ntp_server(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.date.ntp.server","params":{"property":"system.date.ntp.server"}});

    session.call(payload).await
}
///The configured time zone of the system.
pub async fn get_system_date_timezone(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.date.timezone","params":{"property":"system.date.timezone"}});

    session.call(payload).await
}
///Returns true if state is available for this projector
pub async fn get_system_eco_available(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.eco.available","params":{"property":"system.eco.available"}});

    session.call(payload).await
}
///Enable/disable the use of this state. Check if available first.
pub async fn set_system_eco_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "system.eco.enable",
        "params": {
            "property": "system.eco.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///Family name.
pub async fn get_system_familyname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.familyname","params":{"property":"system.familyname"}});

    session.call(payload).await
}
///Firmware version.
pub async fn get_system_firmwareversion(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.firmwareversion","params":{"property":"system.firmwareversion"}});

    session.call(payload).await
}
///The current global health state of the projector. State error means the projector can not safely be operated. Warning means the show can go on, but it is strongly advised to find the cause of the warning and solve it. Normal means the projector is performing normally.
pub async fn get_system_health(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.health","params":{"property":"system.health"}});

    session.call(payload).await
}
///State to transition to when the unit is started
pub async fn set_system_initialstate(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "system.initialstate",
        "params": {
            "property": "system.initialstate",
            "value": value
        }
    });

    session.call(payload).await
}
///Applicability of the license file.
pub async fn get_system_license_applicable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.license.applicable","params":{"property":"system.license.applicable"}});

    session.call(payload).await
}
///Availability of a license file.
pub async fn get_system_license_available(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.license.available","params":{"property":"system.license.available"}});

    session.call(payload).await
}
///A dictionary of options and their values.
pub async fn get_system_license_options(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.license.options","params":{"property":"system.license.options"}});

    session.call(payload).await
}
///Shows if registering the product is mandatory or optional
pub async fn get_system_license_register_mandatory(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.license.register.mandatory","params":{"property":"system.license.register.mandatory"}});

    session.call(payload).await
}
///True when the product has been successfully registered.
pub async fn get_system_license_register_valid(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.license.register.valid","params":{"property":"system.license.register.valid"}});

    session.call(payload).await
}
///Validity of the license file.
pub async fn get_system_license_valid(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.license.valid","params":{"property":"system.license.valid"}});

    session.call(payload).await
}
///Model name.
pub async fn get_system_modelname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.modelname","params":{"property":"system.modelname"}});

    session.call(payload).await
}
///Custom name for this device.
pub async fn get_system_name(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.name","params":{"property":"system.name"}});

    session.call(payload).await
}
///Time (in seconds) to wait in this state before entering lower state.
pub async fn set_system_on_timeout_duration(
    session: &mut PulseSession,
    value: u64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "system.on.timeout.duration",
        "params": {
            "property": "system.on.timeout.duration",
            "value": value
        }
    });

    session.call(payload).await
}
///Enable/disable the timeout.
pub async fn set_system_on_timeout_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "system.on.timeout.enable",
        "params": {
            "property": "system.on.timeout.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///The remaining amount of seconds before the timer will timeout.
pub async fn get_system_on_timeout_remaining(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.on.timeout.remaining","params":{"property":"system.on.timeout.remaining"}});

    session.call(payload).await
}
///List of available operational modes
pub async fn get_system_operationalmodes_availablemodes(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.operationalmodes.availablemodes","params":{"property":"system.operationalmodes.availablemodes"}});

    session.call(payload).await
}
///The currently active operational mode
pub async fn set_system_operationalmodes_mode(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "system.operationalmodes.mode",
        "params": {
            "property": "system.operationalmodes.mode",
            "value": value
        }
    });

    session.call(payload).await
}
///Time (in seconds) to wait in this state before entering lower state.
pub async fn set_system_ready_timeout_duration(
    session: &mut PulseSession,
    value: u64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "system.ready.timeout.duration",
        "params": {
            "property": "system.ready.timeout.duration",
            "value": value
        }
    });

    session.call(payload).await
}
///Enable/disable the timeout.
pub async fn set_system_ready_timeout_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "system.ready.timeout.enable",
        "params": {
            "property": "system.ready.timeout.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///The remaining amount of seconds before the timer will timeout.
pub async fn get_system_ready_timeout_remaining(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.ready.timeout.remaining","params":{"property":"system.ready.timeout.remaining"}});

    session.call(payload).await
}
///Reset progress: [0..100]
pub async fn get_system_resetprogress(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.resetprogress","params":{"property":"system.resetprogress"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_system_resetresult(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.resetresult","params":{"property":"system.resetresult"}});

    session.call(payload).await
}
///Reset status
pub async fn get_system_resetstatus(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.resetstatus","params":{"property":"system.resetstatus"}});

    session.call(payload).await
}
///Serial number.
pub async fn get_system_serialnumber(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.serialnumber","params":{"property":"system.serialnumber"}});

    session.call(payload).await
}
///Returns true if state is available for this projector
pub async fn get_system_standby_available(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.standby.available","params":{"property":"system.standby.available"}});

    session.call(payload).await
}
///Enable/disable the use of this state. Check if available first.
pub async fn set_system_standby_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "system.standby.enable",
        "params": {
            "property": "system.standby.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///Time (in seconds) to wait in this state before entering lower state.
pub async fn set_system_standby_timeout_duration(
    session: &mut PulseSession,
    value: u64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "system.standby.timeout.duration",
        "params": {
            "property": "system.standby.timeout.duration",
            "value": value
        }
    });

    session.call(payload).await
}
///Enable/disable the timeout.
pub async fn set_system_standby_timeout_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "system.standby.timeout.enable",
        "params": {
            "property": "system.standby.timeout.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///The remaining amount of seconds before the timer will timeout.
pub async fn get_system_standby_timeout_remaining(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.standby.timeout.remaining","params":{"property":"system.standby.timeout.remaining"}});

    session.call(payload).await
}
///The current state of the unit
pub async fn get_system_state(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.state","params":{"property":"system.state"}});

    session.call(payload).await
}
///The state the unit is transitioning into
pub async fn get_system_targetstate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"system.targetstate","params":{"property":"system.targetstate"}});

    session.call(payload).await
}
///True and available when the user has end user access privileges.
pub async fn get_ui_access_enduser(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"ui.access.enduser","params":{"property":"ui.access.enduser"}});

    session.call(payload).await
}
///Description not provided
pub async fn set_ui_backlight_state(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.backlight.state",
        "params": {
            "property": "ui.backlight.state",
            "value": value
        }
    });

    session.call(payload).await
}
///The amount of seconds after which the lcd backlight will be switched off when the menu and stealth mode are not active and there is no activity.
pub async fn set_ui_backlight_timeout(
    session: &mut PulseSession,
    value: u64) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.backlight.timeout",
        "params": {
            "property": "ui.backlight.timeout",
            "value": value
        }
    });

    session.call(payload).await
}
///Description not provided
pub async fn get_ui_hasstealthmode(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"ui.hasstealthmode","params":{"property":"ui.hasstealthmode"}});

    session.call(payload).await
}
///The user interface language
pub async fn set_ui_language(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.language",
        "params": {
            "property": "ui.language",
            "value": value
        }
    });

    session.call(payload).await
}
///Drawing commands in the form of a JSON object
pub async fn set_ui_layer_advancedblend_drawing(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.advancedblend.drawing",
        "params": {
            "property": "ui.layer.advancedblend.drawing",
            "value": value
        }
    });

    session.call(payload).await
}
///Enable or disable the layer
pub async fn set_ui_layer_advancedblend_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.advancedblend.enable",
        "params": {
            "property": "ui.layer.advancedblend.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///Color palette that can be used when drawing the blend layer
pub async fn set_ui_layer_advancedblend_palette(
    session: &mut PulseSession,
    value: std::vec::Vec<String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.advancedblend.palette",
        "params": {
            "property": "ui.layer.advancedblend.palette",
            "value": value
        }
    });

    session.call(payload).await
}
///The edge color, e.g '#ffff00' or 'rgba(255,255,0,0.5)
pub async fn set_ui_layer_basicblacklevel_color(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.basicblacklevel.color",
        "params": {
            "property": "ui.layer.basicblacklevel.color",
            "value": value
        }
    });

    session.call(payload).await
}
///Enable or disable the layer
pub async fn set_ui_layer_basicblacklevel_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.basicblacklevel.enable",
        "params": {
            "property": "ui.layer.basicblacklevel.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///Toggle edge selection
pub async fn set_ui_layer_basicblacklevel_selection(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.basicblacklevel.selection",
        "params": {
            "property": "ui.layer.basicblacklevel.selection",
            "value": value
        }
    });

    session.call(payload).await
}
///The color to use for selected edges, e.g '#ff0000' or 'rgba(255,0,0,0.5)'
pub async fn set_ui_layer_basicblacklevel_selectioncolor(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.basicblacklevel.selectioncolor",
        "params": {
            "property": "ui.layer.basicblacklevel.selectioncolor",
            "value": value
        }
    });

    session.call(payload).await
}
///The edge color, e.g '#ffff00' or 'rgba(255,255,0,0.5)
pub async fn set_ui_layer_basicblend_color(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.basicblend.color",
        "params": {
            "property": "ui.layer.basicblend.color",
            "value": value
        }
    });

    session.call(payload).await
}
///Enable or disable the layer
pub async fn set_ui_layer_basicblend_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.basicblend.enable",
        "params": {
            "property": "ui.layer.basicblend.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///Toggle edge selection
pub async fn set_ui_layer_basicblend_selection(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.basicblend.selection",
        "params": {
            "property": "ui.layer.basicblend.selection",
            "value": value
        }
    });

    session.call(payload).await
}
///The color to use for selected edges, e.g '#ff0000' or 'rgba(255,0,0,0.5)'
pub async fn set_ui_layer_basicblend_selectioncolor(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.basicblend.selectioncolor",
        "params": {
            "property": "ui.layer.basicblend.selectioncolor",
            "value": value
        }
    });

    session.call(payload).await
}
///(Optional) Show/hide a border/outline of the screen
pub async fn set_ui_layer_blank_border(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.blank.border",
        "params": {
            "property": "ui.layer.blank.border",
            "value": value
        }
    });

    session.call(payload).await
}
///(Optional) Specify the border color, e.g '#ff0000' or 'rgba(255,0,0,0.75)'
pub async fn set_ui_layer_blank_bordercolor(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.blank.bordercolor",
        "params": {
            "property": "ui.layer.blank.bordercolor",
            "value": value
        }
    });

    session.call(payload).await
}
///Enable or disable the layer. When enabled, the screen will be covered in black.
pub async fn set_ui_layer_blank_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.blank.enable",
        "params": {
            "property": "ui.layer.blank.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///A single UTF character from the Barco-Icons font set.
pub async fn set_ui_layer_blank_icon(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.blank.icon",
        "params": {
            "property": "ui.layer.blank.icon",
            "value": value
        }
    });

    session.call(payload).await
}
///Used only for persisting whether or not to show the layer
pub async fn set_ui_layer_blank_show(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.blank.show",
        "params": {
            "property": "ui.layer.blank.show",
            "value": value
        }
    });

    session.call(payload).await
}
///Show or hide the text
pub async fn set_ui_layer_blank_showtext(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.blank.showtext",
        "params": {
            "property": "ui.layer.blank.showtext",
            "value": value
        }
    });

    session.call(payload).await
}
///(Optional) Specify a text to show at the center of the screen
pub async fn set_ui_layer_blank_text(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.blank.text",
        "params": {
            "property": "ui.layer.blank.text",
            "value": value
        }
    });

    session.call(payload).await
}
///The color to use for unselected corners, e.g '#ff0000' or 'rgba(255,0,0,0.75)'
pub async fn set_ui_layer_fourcorner_cornercolor(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.fourcorner.cornercolor",
        "params": {
            "property": "ui.layer.fourcorner.cornercolor",
            "value": value
        }
    });

    session.call(payload).await
}
///Enable or disable the layer
pub async fn set_ui_layer_fourcorner_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.fourcorner.enable",
        "params": {
            "property": "ui.layer.fourcorner.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///The line color, e.g '#ffff00' or 'rgba(255,255,0,0.5)'
pub async fn set_ui_layer_fourcorner_linecolor(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.fourcorner.linecolor",
        "params": {
            "property": "ui.layer.fourcorner.linecolor",
            "value": value
        }
    });

    session.call(payload).await
}
///Show or hide lines between the corners
pub async fn set_ui_layer_fourcorner_lines(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.fourcorner.lines",
        "params": {
            "property": "ui.layer.fourcorner.lines",
            "value": value
        }
    });

    session.call(payload).await
}
///Toggle corner selection
pub async fn set_ui_layer_fourcorner_selection(
    session: &mut PulseSession,
    value: std::collections::HashMap<String, String>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.fourcorner.selection",
        "params": {
            "property": "ui.layer.fourcorner.selection",
            "value": value
        }
    });

    session.call(payload).await
}
///The color to use for selected corners, e.g '#ff0000' or 'rgba(255,0,0,0.75)'
pub async fn set_ui_layer_fourcorner_selectioncolor(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.fourcorner.selectioncolor",
        "params": {
            "property": "ui.layer.fourcorner.selectioncolor",
            "value": value
        }
    });

    session.call(payload).await
}
///Default color for grid points, e.g '#ff0000' or 'rgba(0,0,255,0.5)'
pub async fn set_ui_layer_grid_color(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.grid.color",
        "params": {
            "property": "ui.layer.grid.color",
            "value": value
        }
    });

    session.call(payload).await
}
///Enable or disable the layer
pub async fn set_ui_layer_grid_enable(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.grid.enable",
        "params": {
            "property": "ui.layer.grid.enable",
            "value": value
        }
    });

    session.call(payload).await
}
///[DEPRECATED] Use ShowLines instead. This is for backwards compability
pub async fn set_ui_layer_grid_lines(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.grid.lines",
        "params": {
            "property": "ui.layer.grid.lines",
            "value": value
        }
    });

    session.call(payload).await
}
///List of row,column and color triplets for marking points in the grid. The color is specified as '#ff00ff' or 'rgba(0,255,255,0.75)'
pub async fn set_ui_layer_grid_mark(
    session: &mut PulseSession,
    value: std::vec::Vec<std::collections::HashMap<String, String>>) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.grid.mark",
        "params": {
            "property": "ui.layer.grid.mark",
            "value": value
        }
    });

    session.call(payload).await
}
///Number of grid points
pub async fn set_ui_layer_grid_points(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.grid.points",
        "params": {
            "property": "ui.layer.grid.points",
            "value": value
        }
    });

    session.call(payload).await
}
///Toggle drawing lines between grid points
pub async fn set_ui_layer_grid_showlines(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.grid.showlines",
        "params": {
            "property": "ui.layer.grid.showlines",
            "value": value
        }
    });

    session.call(payload).await
}
///Toggle drawing grid points
pub async fn set_ui_layer_grid_showpoints(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.layer.grid.showpoints",
        "params": {
            "property": "ui.layer.grid.showpoints",
            "value": value
        }
    });

    session.call(payload).await
}
///Preferred unit for display of lengths and distances
pub async fn set_ui_length(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.length",
        "params": {
            "property": "ui.length",
            "value": value
        }
    });

    session.call(payload).await
}
///Show or hide the lens menu
pub async fn set_ui_lensmenu(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.lensmenu",
        "params": {
            "property": "ui.lensmenu",
            "value": value
        }
    });

    session.call(payload).await
}
///Preferred unit for luminance
pub async fn set_ui_luminance(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.luminance",
        "params": {
            "property": "ui.luminance",
            "value": value
        }
    });

    session.call(payload).await
}
///Show or hide the menu
pub async fn set_ui_menu(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.menu",
        "params": {
            "property": "ui.menu",
            "value": value
        }
    });

    session.call(payload).await
}
///Placement of menu related to full screen.
pub async fn set_ui_menuposition(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.menuposition",
        "params": {
            "property": "ui.menuposition",
            "value": value
        }
    });

    session.call(payload).await
}
///The persistent message to show on the screen. Rich text allowed
pub async fn get_ui_message(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"ui.message","params":{"property":"ui.message"}});

    session.call(payload).await
}
///Placement of the message relative to full screen
pub async fn get_ui_messageposition(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"ui.messageposition","params":{"property":"ui.messageposition"}});

    session.call(payload).await
}
///Minimize the menu when it is enabled
pub async fn set_ui_minimize(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.minimize",
        "params": {
            "property": "ui.minimize",
            "value": value
        }
    });

    session.call(payload).await
}
///Enable or disable on screen display
pub async fn set_ui_osd(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.osd",
        "params": {
            "property": "ui.osd",
            "value": value
        }
    });

    session.call(payload).await
}
///Show or hide the pattern menu shortcut
pub async fn set_ui_patternmenu(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.patternmenu",
        "params": {
            "property": "ui.patternmenu",
            "value": value
        }
    });

    session.call(payload).await
}
///When true, a dialog shows info about powering down
pub async fn set_ui_poweroffhint(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.poweroffhint",
        "params": {
            "property": "ui.poweroffhint",
            "value": value
        }
    });

    session.call(payload).await
}
///The unit.
pub async fn get_ui_sensors_airpressure_unit(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"ui.sensors.airpressure.unit","params":{"property":"ui.sensors.airpressure.unit"}});

    session.call(payload).await
}
///The value.
pub async fn get_ui_sensors_airpressure_value(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"ui.sensors.airpressure.value","params":{"property":"ui.sensors.airpressure.value"}});

    session.call(payload).await
}
///The unit.
pub async fn get_ui_sensors_ambienttemperature_unit(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"ui.sensors.ambienttemperature.unit","params":{"property":"ui.sensors.ambienttemperature.unit"}});

    session.call(payload).await
}
///The value.
pub async fn get_ui_sensors_ambienttemperature_value(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"ui.sensors.ambienttemperature.value","params":{"property":"ui.sensors.ambienttemperature.value"}});

    session.call(payload).await
}
///The unit.
pub async fn get_ui_sensors_humidity_unit(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"ui.sensors.humidity.unit","params":{"property":"ui.sensors.humidity.unit"}});

    session.call(payload).await
}
///The value.
pub async fn get_ui_sensors_humidity_value(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"ui.sensors.humidity.value","params":{"property":"ui.sensors.humidity.value"}});

    session.call(payload).await
}
///The unit.
pub async fn get_ui_sensors_mainsvoltage_unit(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"ui.sensors.mainsvoltage.unit","params":{"property":"ui.sensors.mainsvoltage.unit"}});

    session.call(payload).await
}
///The value.
pub async fn get_ui_sensors_mainsvoltage_value(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"ui.sensors.mainsvoltage.value","params":{"property":"ui.sensors.mainsvoltage.value"}});

    session.call(payload).await
}
///Show a persistent message on the screen
pub async fn get_ui_showmessage(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"ui.showmessage","params":{"property":"ui.showmessage"}});

    session.call(payload).await
}
///Show or hide the input source shortcut menu
pub async fn set_ui_sourcemenu(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.sourcemenu",
        "params": {
            "property": "ui.sourcemenu",
            "value": value
        }
    });

    session.call(payload).await
}
///Show/hide the source signal information popup
pub async fn set_ui_sourcesignal(
    session: &mut PulseSession,
    value: bool) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.sourcesignal",
        "params": {
            "property": "ui.sourcesignal",
            "value": value
        }
    });

    session.call(payload).await
}
///Placement of the source signal information
pub async fn set_ui_sourcesignalposition(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.sourcesignalposition",
        "params": {
            "property": "ui.sourcesignalposition",
            "value": value
        }
    });

    session.call(payload).await
}
///Value to indicate if the splash screen should be shown
pub async fn get_ui_splashscreen_show(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"ui.splashscreen.show","params":{"property":"ui.splashscreen.show"}});

    session.call(payload).await
}
///Time (in seconds) the splash screen is shown on startup
pub async fn get_ui_splashscreen_timeoutseconds(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"ui.splashscreen.timeoutseconds","params":{"property":"ui.splashscreen.timeoutseconds"}});

    session.call(payload).await
}
///When the projector is in stealth mode, all controllable LEDs are switched off.
pub async fn set_ui_stealthmode(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.stealthmode",
        "params": {
            "property": "ui.stealthmode",
            "value": value
        }
    });

    session.call(payload).await
}
///Preferred unit for display of temperature values
pub async fn set_ui_temperature(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.temperature",
        "params": {
            "property": "ui.temperature",
            "value": value
        }
    });

    session.call(payload).await
}
///The theme setting of the user interface.
pub async fn set_ui_theme(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.theme",
        "params": {
            "property": "ui.theme",
            "value": value
        }
    });

    session.call(payload).await
}
///Enables or disables the touchscreen test.
pub async fn get_ui_touchscreen_mode(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"ui.touchscreen.mode","params":{"property":"ui.touchscreen.mode"}});

    session.call(payload).await
}
///Current status of the touchscreen text
pub async fn set_ui_touchscreen_status(
    session: &mut PulseSession,
    value: String) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "property.set",
        "id": "ui.touchscreen.status",
        "params": {
            "property": "ui.touchscreen.status",
            "value": value
        }
    });

    session.call(payload).await
}
///Description not provided
pub async fn get_user_admin_activesessioncount(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.admin.activesessioncount","params":{"property":"user.admin.activesessioncount"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_admin_enabled(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.admin.enabled","params":{"property":"user.admin.enabled"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_admin_group(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.admin.group","params":{"property":"user.admin.group"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_admin_publickey(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.admin.publickey","params":{"property":"user.admin.publickey"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_admin_username(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.admin.username","params":{"property":"user.admin.username"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_admin_userslug(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.admin.userslug","params":{"property":"user.admin.userslug"}});

    session.call(payload).await
}
///Require authentication of all users
pub async fn get_user_authenticationrequired(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.authenticationrequired","params":{"property":"user.authenticationrequired"}});

    session.call(payload).await
}
///The available user groups
pub async fn get_user_availablegroups(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.availablegroups","params":{"property":"user.availablegroups"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_currentuser_activesessioncount(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.currentuser.activesessioncount","params":{"property":"user.currentuser.activesessioncount"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_currentuser_enabled(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.currentuser.enabled","params":{"property":"user.currentuser.enabled"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_currentuser_group(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.currentuser.group","params":{"property":"user.currentuser.group"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_currentuser_publickey(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.currentuser.publickey","params":{"property":"user.currentuser.publickey"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_currentuser_username(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.currentuser.username","params":{"property":"user.currentuser.username"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_currentuser_userslug(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.currentuser.userslug","params":{"property":"user.currentuser.userslug"}});

    session.call(payload).await
}
///The list of all users
pub async fn get_user_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.list","params":{"property":"user.list"}});

    session.call(payload).await
}
///Allow authenticating users with pin code
pub async fn get_user_pincodeauthenticationenabled(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.pincodeauthenticationenabled","params":{"property":"user.pincodeauthenticationenabled"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_poweruser_activesessioncount(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.poweruser.activesessioncount","params":{"property":"user.poweruser.activesessioncount"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_poweruser_enabled(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.poweruser.enabled","params":{"property":"user.poweruser.enabled"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_poweruser_group(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.poweruser.group","params":{"property":"user.poweruser.group"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_poweruser_publickey(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.poweruser.publickey","params":{"property":"user.poweruser.publickey"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_poweruser_username(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.poweruser.username","params":{"property":"user.poweruser.username"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_poweruser_userslug(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.poweruser.userslug","params":{"property":"user.poweruser.userslug"}});

    session.call(payload).await
}
///Challenge for resetting the administrator user
pub async fn get_user_resetadministratorchallenge(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.resetadministratorchallenge","params":{"property":"user.resetadministratorchallenge"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_testuser_activesessioncount(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.testuser.activesessioncount","params":{"property":"user.testuser.activesessioncount"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_testuser_enabled(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.testuser.enabled","params":{"property":"user.testuser.enabled"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_testuser_group(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.testuser.group","params":{"property":"user.testuser.group"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_testuser_publickey(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.testuser.publickey","params":{"property":"user.testuser.publickey"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_testuser_username(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.testuser.username","params":{"property":"user.testuser.username"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_testuser_userslug(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.testuser.userslug","params":{"property":"user.testuser.userslug"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_user_activesessioncount(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.user.activesessioncount","params":{"property":"user.user.activesessioncount"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_user_enabled(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.user.enabled","params":{"property":"user.user.enabled"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_user_group(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.user.group","params":{"property":"user.user.group"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_user_publickey(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.user.publickey","params":{"property":"user.user.publickey"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_user_username(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.user.username","params":{"property":"user.user.username"}});

    session.call(payload).await
}
///Description not provided
pub async fn get_user_user_userslug(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"property.get","id":"user.user.userslug","params":{"property":"user.user.userslug"}});

    session.call(payload).await
}
