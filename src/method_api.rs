
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
/// custom pulse method api call
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
///[DEPRECATED] Authenticate using pin code
pub async fn set_authenticate(
    session: &mut PulseSession,
    code: u64,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "authenticate",
        "id": "authenticate",
        "params": {"code": code,}
    });

    session.call(payload).await
}
///Request a new token for the currently authenticated user
pub async fn set_generatenewtoken(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"generatenewtoken","id":"generatenewtoken","params":{}});

    session.call(payload).await
}
///Get the challenge for key-based authentication
pub async fn set_getchallenge(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"getchallenge","id":"getchallenge","params":{}});

    session.call(payload).await
}
///Introspect the API for a specified object path
pub async fn set_introspect(
    session: &mut PulseSession,
    object: String,descriptions: bool,recursive: bool,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "introspect",
        "id": "introspect",
        "params": {"object": object,"descriptions": descriptions,"recursive": recursive,}
    });

    session.call(payload).await
}
///Login using username and signed challenge (set token true to receive a token)
pub async fn set_login(
    session: &mut PulseSession,
    signature: String,token: bool,username: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "login",
        "id": "login",
        "params": {"signature": signature,"token": token,"username": username,}
    });

    session.call(payload).await
}
///Login using username and password (set token true to receive a token)
pub async fn set_login_basic(
    session: &mut PulseSession,
    password: String,token: bool,username: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "login",
        "id": "login_basic",
        "params": {"password": password,"token": token,"username": username,}
    });

    session.call(payload).await
}
///Login using token
pub async fn set_login_token(
    session: &mut PulseSession,
    token: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "login",
        "id": "login_token",
        "params": {"token": token,}
    });

    session.call(payload).await
}
///Login using code
pub async fn set_login_code(
    session: &mut PulseSession,
    token: bool,code: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "login",
        "id": "login_code",
        "params": {"token": token,"code": code,}
    });

    session.call(payload).await
}
///De-authenticate
pub async fn set_logout(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"logout","id":"logout","params":{}});

    session.call(payload).await
}
///Return a list of available channel names
pub async fn set_dmx_listchannels(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"dmx.listchannels","id":"dmx.listchannels","params":{}});

    session.call(payload).await
}
///Return a list of all modes
pub async fn set_dmx_listmodes(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"dmx.listmodes","id":"dmx.listmodes","params":{}});

    session.call(payload).await
}
///Description not provided
pub async fn set_environment_getalarminfo(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"environment.getalarminfo","id":"environment.getalarminfo","params":{}});

    session.call(payload).await
}
///Description not provided
pub async fn set_environment_getcontrolblocks(
    session: &mut PulseSession,
    valuetype: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "environment.getcontrolblocks",
        "id": "environment.getcontrolblocks",
        "params": {"valuetype": valuetype,}
    });

    session.call(payload).await
}
///List of possible GPO modes.
pub async fn set_gpio_gpo_trigger1_listavailablemodes(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"gpio.gpo.trigger1.listavailablemodes","id":"gpio.gpo.trigger1.listavailablemodes","params":{}});

    session.call(payload).await
}
///Single toggle value of output
pub async fn set_gpio_gpo_trigger1_pulse(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"gpio.gpo.trigger1.pulse","id":"gpio.gpo.trigger1.pulse","params":{}});

    session.call(payload).await
}
///List of possible GPO modes.
pub async fn set_gpio_gpo_trigger2_listavailablemodes(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"gpio.gpo.trigger2.listavailablemodes","id":"gpio.gpo.trigger2.listavailablemodes","params":{}});

    session.call(payload).await
}
///Single toggle value of output
pub async fn set_gpio_gpo_trigger2_pulse(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"gpio.gpo.trigger2.pulse","id":"gpio.gpo.trigger2.pulse","params":{}});

    session.call(payload).await
}
///List the names of all General Purpose Inputs
pub async fn set_gpio_listinputs(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"gpio.listinputs","id":"gpio.listinputs","params":{}});

    session.call(payload).await
}
///List the names of all General Purpose Outputs
pub async fn set_gpio_listoutputs(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"gpio.listoutputs","id":"gpio.listoutputs","params":{}});

    session.call(payload).await
}
///Engage CLO at the current light level
pub async fn set_illumination_clo_engage(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"illumination.clo.engage","id":"illumination.clo.engage","params":{}});

    session.call(payload).await
}
///List possible BrilliantColor modes.
pub async fn set_image_brilliantcolor_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.brilliantcolor.list","id":"image.brilliantcolor.list","params":{}});

    session.call(payload).await
}
///Description not provided
pub async fn set_image_color_p7_custom_copypresettocustom(
    session: &mut PulseSession,
    presetname: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "image.color.p7.custom.copypresettocustom",
        "id": "image.color.p7.custom.copypresettocustom",
        "params": {"presetname": presetname,}
    });

    session.call(payload).await
}
///Create a balanced colorspace from the current primaries
pub async fn set_image_color_p7_custom_makebalanced(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.color.p7.custom.makebalanced","id":"image.color.p7.custom.makebalanced","params":{}});

    session.call(payload).await
}
///Reset preset back to its default values
pub async fn set_image_color_p7_custom_resetpreset(
    session: &mut PulseSession,
    presetname: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "image.color.p7.custom.resetpreset",
        "id": "image.color.p7.custom.resetpreset",
        "params": {"presetname": presetname,}
    });

    session.call(payload).await
}
///Description not provided
pub async fn set_image_color_p7_custom_resettonative(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.color.p7.custom.resettonative","id":"image.color.p7.custom.resettonative","params":{}});

    session.call(payload).await
}
///Change to the next RGB mode. Lets you cycle through all possible modes.
pub async fn set_image_color_rgbmode_nextrgbmode(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.color.rgbmode.nextrgbmode","id":"image.color.rgbmode.nextrgbmode","params":{}});

    session.call(payload).await
}
///List system EDIDs available for this connector
pub async fn set_image_connector_displayport_edid_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.connector.displayport.edid.list","id":"image.connector.displayport.edid.list","params":{}});

    session.call(payload).await
}
///List system EDIDs available for this connector
pub async fn set_image_connector_hdmi_edid_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.connector.hdmi.edid.list","id":"image.connector.hdmi.edid.list","params":{}});

    session.call(payload).await
}
///Description not provided
pub async fn set_image_connector_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.connector.list","id":"image.connector.list","params":{}});

    session.call(payload).await
}
///List possible display modes.
pub async fn set_image_display_listdisplaymodes(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.display.listdisplaymodes","id":"image.display.listdisplaymodes","params":{}});

    session.call(payload).await
}
///List all available orientation modes
pub async fn set_image_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.list","id":"image.list","params":{}});

    session.call(payload).await
}
///List all available gamma types.
pub async fn set_image_listgammatypes(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.listgammatypes","id":"image.listgammatypes","params":{}});

    session.call(payload).await
}
///Returns the four boxes describing the black level edges.
pub async fn set_image_processing_blacklevel_basicblacklevel_getblacklevelarea(
    session: &mut PulseSession,
    resolution_width: std::collections::HashMap<String, String>,resolution_height: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "image.processing.blacklevel.basicblacklevel.getblacklevelarea",
        "id": "image.processing.blacklevel.basicblacklevel.getblacklevelarea",
        "params": {"resolution_width": resolution_width,"resolution_height": resolution_height,}
    });

    session.call(payload).await
}
///Returns the four boxes describing the black level edges, after warp.
pub async fn set_image_processing_blacklevel_basicblacklevel_getwarpedblacklevelarea(
    session: &mut PulseSession,
    resolution_width: std::collections::HashMap<String, String>,resolution_height: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "image.processing.blacklevel.basicblacklevel.getwarpedblacklevelarea",
        "id": "image.processing.blacklevel.basicblacklevel.getwarpedblacklevelarea",
        "params": {"resolution_width": resolution_width,"resolution_height": resolution_height,}
    });

    session.call(payload).await
}
///Deletes afile with the given name.
pub async fn set_image_processing_blacklevel_file_delete(
    session: &mut PulseSession,
    filename: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "image.processing.blacklevel.file.delete",
        "id": "image.processing.blacklevel.file.delete",
        "params": {"filename": filename,}
    });

    session.call(payload).await
}
///Returns a list of available black level correctionfiles
pub async fn set_image_processing_blacklevel_file_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.processing.blacklevel.file.list","id":"image.processing.blacklevel.file.list","params":{}});

    session.call(payload).await
}
///Returns the four boxes describing the blend edges.
pub async fn set_image_processing_blend_basicblend_getblendarea(
    session: &mut PulseSession,
    resolution_width: std::collections::HashMap<String, String>,resolution_height: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "image.processing.blend.basicblend.getblendarea",
        "id": "image.processing.blend.basicblend.getblendarea",
        "params": {"resolution_width": resolution_width,"resolution_height": resolution_height,}
    });

    session.call(payload).await
}
///Returns the four boxes describing the blend edges, after warp.
pub async fn set_image_processing_blend_basicblend_getwarpedblendarea(
    session: &mut PulseSession,
    resolution_width: std::collections::HashMap<String, String>,resolution_height: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "image.processing.blend.basicblend.getwarpedblendarea",
        "id": "image.processing.blend.basicblend.getwarpedblendarea",
        "params": {"resolution_width": resolution_width,"resolution_height": resolution_height,}
    });

    session.call(payload).await
}
///Deletes afile with the given name.
pub async fn set_image_processing_blend_file_delete(
    session: &mut PulseSession,
    filename: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "image.processing.blend.file.delete",
        "id": "image.processing.blend.file.delete",
        "params": {"filename": filename,}
    });

    session.call(payload).await
}
///Returns a list of available blendfiles
pub async fn set_image_processing_blend_file_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.processing.blend.file.list","id":"image.processing.blend.file.list","params":{}});

    session.call(payload).await
}
///Deletes afile with the given name.
pub async fn set_image_processing_warp_file_delete(
    session: &mut PulseSession,
    filename: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "image.processing.warp.file.delete",
        "id": "image.processing.warp.file.delete",
        "params": {"filename": filename,}
    });

    session.call(payload).await
}
///Returns a list of available warpfiles
pub async fn set_image_processing_warp_file_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.processing.warp.file.list","id":"image.processing.warp.file.list","params":{}});

    session.call(payload).await
}
///Get the corners scaled to the given resolution
pub async fn set_image_processing_warp_fourcorners_getscaledcorners(
    session: &mut PulseSession,
    resolution: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "image.processing.warp.fourcorners.getscaledcorners",
        "id": "image.processing.warp.fourcorners.getscaledcorners",
        "params": {"resolution": resolution,}
    });

    session.call(payload).await
}
///Takes an array of points and returns their warped equivalents.
pub async fn set_image_processing_warp_warpscaledpoints(
    session: &mut PulseSession,
    points: std::vec::Vec<std::collections::HashMap<String, String>>,resolution: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "image.processing.warp.warpscaledpoints",
        "id": "image.processing.warp.warpscaledpoints",
        "params": {"points": points,"resolution": resolution,}
    });

    session.call(payload).await
}
///Get the current grid points as normalized and relative
pub async fn set_image_processing_warpgrid_getgrid(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.processing.warpgrid.getgrid","id":"image.processing.warpgrid.getgrid","params":{}});

    session.call(payload).await
}
///Description not provided
pub async fn set_image_processing_warpgrid_getgridsize(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.processing.warpgrid.getgridsize","id":"image.processing.warpgrid.getgridsize","params":{}});

    session.call(payload).await
}
///Get the current grid scaled to the given resolution
pub async fn set_image_processing_warpgrid_getscaledgrid(
    session: &mut PulseSession,
    resolution: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "image.processing.warpgrid.getscaledgrid",
        "id": "image.processing.warpgrid.getscaledgrid",
        "params": {"resolution": resolution,}
    });

    session.call(payload).await
}
///List all possible resolutions.
pub async fn set_image_resolution_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.resolution.list","id":"image.resolution.list","params":{}});

    session.call(payload).await
}
///Get all connectors that are assigned to this source with their layout position
pub async fn set_image_source_displayport_listconnectors(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.source.displayport.listconnectors","id":"image.source.displayport.listconnectors","params":{}});

    session.call(payload).await
}
///Get all connectors that are assigned to this source with their layout position
pub async fn set_image_source_hdmi_listconnectors(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.source.hdmi.listconnectors","id":"image.source.hdmi.listconnectors","params":{}});

    session.call(payload).await
}
///List all available sources
pub async fn set_image_source_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.source.list","id":"image.source.list","params":{}});

    session.call(payload).await
}
///List all possible darktime values (in us).
pub async fn set_image_stereo_listdarktime(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.stereo.listdarktime","id":"image.stereo.listdarktime","params":{}});

    session.call(payload).await
}
///Deletes afile with the given name.
pub async fn set_image_testpattern_file_delete(
    session: &mut PulseSession,
    filename: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "image.testpattern.file.delete",
        "id": "image.testpattern.file.delete",
        "params": {"filename": filename,}
    });

    session.call(payload).await
}
///Get a list of available custom uploaded patterns
pub async fn set_image_testpattern_file_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.testpattern.file.list","id":"image.testpattern.file.list","params":{}});

    session.call(payload).await
}
///Get the properties of a pattern
pub async fn set_image_testpattern_getproperties(
    session: &mut PulseSession,
    id: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "image.testpattern.getproperties",
        "id": "image.testpattern.getproperties",
        "params": {"id": id,}
    });

    session.call(payload).await
}
///Return the list of possible testpatterns to iterate.
pub async fn set_image_testpattern_iterator_calibration_listpossiblepatterns(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.testpattern.iterator.calibration.listpossiblepatterns","id":"image.testpattern.iterator.calibration.listpossiblepatterns","params":{}});

    session.call(payload).await
}
///Show the next testpattern in the list.
pub async fn set_image_testpattern_iterator_calibration_next(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.testpattern.iterator.calibration.next","id":"image.testpattern.iterator.calibration.next","params":{}});

    session.call(payload).await
}
///Return the list of possible testpatterns to iterate.
pub async fn set_image_testpattern_iterator_iris_listpossiblepatterns(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.testpattern.iterator.iris.listpossiblepatterns","id":"image.testpattern.iterator.iris.listpossiblepatterns","params":{}});

    session.call(payload).await
}
///Show the next testpattern in the list.
pub async fn set_image_testpattern_iterator_iris_next(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.testpattern.iterator.iris.next","id":"image.testpattern.iterator.iris.next","params":{}});

    session.call(payload).await
}
///Return the list of possible testpatterns to iterate.
pub async fn set_image_testpattern_iterator_optics_listpossiblepatterns(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.testpattern.iterator.optics.listpossiblepatterns","id":"image.testpattern.iterator.optics.listpossiblepatterns","params":{}});

    session.call(payload).await
}
///Show the next testpattern in the list.
pub async fn set_image_testpattern_iterator_optics_next(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.testpattern.iterator.optics.next","id":"image.testpattern.iterator.optics.next","params":{}});

    session.call(payload).await
}
///Return the list of possible testpatterns to iterate.
pub async fn set_image_testpattern_iterator_shift_listpossiblepatterns(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.testpattern.iterator.shift.listpossiblepatterns","id":"image.testpattern.iterator.shift.listpossiblepatterns","params":{}});

    session.call(payload).await
}
///Show the next testpattern in the list.
pub async fn set_image_testpattern_iterator_shift_next(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.testpattern.iterator.shift.next","id":"image.testpattern.iterator.shift.next","params":{}});

    session.call(payload).await
}
///Return the list of possible testpatterns to iterate.
pub async fn set_image_testpattern_iterator_stereo_listpossiblepatterns(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.testpattern.iterator.stereo.listpossiblepatterns","id":"image.testpattern.iterator.stereo.listpossiblepatterns","params":{}});

    session.call(payload).await
}
///Show the next testpattern in the list.
pub async fn set_image_testpattern_iterator_stereo_next(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.testpattern.iterator.stereo.next","id":"image.testpattern.iterator.stereo.next","params":{}});

    session.call(payload).await
}
///Return the list of possible testpatterns to iterate.
pub async fn set_image_testpattern_iterator_xpr_listpossiblepatterns(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.testpattern.iterator.xpr.listpossiblepatterns","id":"image.testpattern.iterator.xpr.listpossiblepatterns","params":{}});

    session.call(payload).await
}
///Show the next testpattern in the list.
pub async fn set_image_testpattern_iterator_xpr_next(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.testpattern.iterator.xpr.next","id":"image.testpattern.iterator.xpr.next","params":{}});

    session.call(payload).await
}
///Get a list of available patterns
pub async fn set_image_testpattern_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.testpattern.list","id":"image.testpattern.list","params":{}});

    session.call(payload).await
}
///Get a list of available internal patterns
pub async fn set_image_testpattern_listinternal(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.testpattern.listinternal","id":"image.testpattern.listinternal","params":{}});

    session.call(payload).await
}
///Get a list of available NoSignal patterns
pub async fn set_image_testpattern_nosignal_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.testpattern.nosignal.list","id":"image.testpattern.nosignal.list","params":{}});

    session.call(payload).await
}
///Set the properties of a pattern
pub async fn set_image_testpattern_setproperties(
    session: &mut PulseSession,
    value: String,id: String,key: String,properties: std::vec::Vec<std::collections::HashMap<String, String>>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "image.testpattern.setproperties",
        "id": "image.testpattern.setproperties",
        "params": {"value": value,"id": id,"key": key,"properties": properties,}
    });

    session.call(payload).await
}
///Toggle display of selected test pattern on/off
pub async fn set_image_testpattern_toggle(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.testpattern.toggle","id":"image.testpattern.toggle","params":{}});

    session.call(payload).await
}
///List all available windows
pub async fn set_image_window_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"image.window.list","id":"image.window.list","params":{}});

    session.call(payload).await
}
///Create and send a synthetic key event
pub async fn set_keydispatcher_postevent(
    session: &mut PulseSession,
    key: String,eventtype: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "keydispatcher.postevent",
        "id": "keydispatcher.postevent",
        "params": {"key": key,"eventtype": eventtype,}
    });

    session.call(payload).await
}
///Activates the LEDS when enabled and restarts the LED timeout counter
pub async fn set_led_activity(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"led.activity","id":"led.activity","params":{}});

    session.call(payload).await
}
///Description not provided
pub async fn set_led_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"led.list","id":"led.list","params":{}});

    session.call(payload).await
}
///Mutes the LEDS.
pub async fn set_led_muteall(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"led.muteall","id":"led.muteall","params":{}});

    session.call(payload).await
}
///Description not provided
pub async fn set_lightmeasurement_getlightoutput(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"lightmeasurement.getlightoutput","id":"lightmeasurement.getlightoutput","params":{}});

    session.call(payload).await
}
///List of logical device id, e.g: 'wired1', 'wifi1'
pub async fn set_network_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"network.list","id":"network.list","params":{}});

    session.call(payload).await
}
///Retrieve all active notifications for the given severity
pub async fn set_notification_caution_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"notification.caution.list","id":"notification.caution.list","params":{}});

    session.call(payload).await
}
///Dismiss the notification with the specified id
pub async fn set_notification_dismiss(
    session: &mut PulseSession,
    response: String,id: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "notification.dismiss",
        "id": "notification.dismiss",
        "params": {"response": response,"id": id,}
    });

    session.call(payload).await
}
///Retrieve all active notifications for the given severity
pub async fn set_notification_error_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"notification.error.list","id":"notification.error.list","params":{}});

    session.call(payload).await
}
///Retrieve all active notifications for the given severity
pub async fn set_notification_info_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"notification.info.list","id":"notification.info.list","params":{}});

    session.call(payload).await
}
///List all active notifications
pub async fn set_notification_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"notification.list","id":"notification.list","params":{}});

    session.call(payload).await
}
///Get a list of suppressed notification codes
pub async fn set_notification_listsuppressed(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"notification.listsuppressed","id":"notification.listsuppressed","params":{}});

    session.call(payload).await
}
///List received notifications
pub async fn set_notification_log(
    session: &mut PulseSession,
    start: std::collections::HashMap<String, String>,count: std::collections::HashMap<String, String>,minimumseverity: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "notification.log",
        "id": "notification.log",
        "params": {"start": start,"count": count,"minimumseverity": minimumseverity,}
    });

    session.call(payload).await
}
///Add a notification code to suppress (log but do not show on the LCD/OSD)
pub async fn set_notification_suppress(
    session: &mut PulseSession,
    code: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "notification.suppress",
        "id": "notification.suppress",
        "params": {"code": code,}
    });

    session.call(payload).await
}
///No longer suppress a certain notification code
pub async fn set_notification_unsuppress(
    session: &mut PulseSession,
    code: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "notification.unsuppress",
        "id": "notification.unsuppress",
        "params": {"code": code,}
    });

    session.call(payload).await
}
///No longer suppress any notification codes
pub async fn set_notification_unsuppressall(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"notification.unsuppressall","id":"notification.unsuppressall","params":{}});

    session.call(payload).await
}
///Retrieve all active notifications for the given severity
pub async fn set_notification_warning_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"notification.warning.list","id":"notification.warning.list","params":{}});

    session.call(payload).await
}
///Calibrate motor
pub async fn set_optics_focus_calibrate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"optics.focus.calibrate","id":"optics.focus.calibrate","params":{}});

    session.call(payload).await
}
///Run forward
pub async fn set_optics_focus_runforward(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"optics.focus.runforward","id":"optics.focus.runforward","params":{}});

    session.call(payload).await
}
///Run forward for X milliseconds
pub async fn set_optics_focus_runforwardtime(
    session: &mut PulseSession,
    milliseconds: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "optics.focus.runforwardtime",
        "id": "optics.focus.runforwardtime",
        "params": {"milliseconds": milliseconds,}
    });

    session.call(payload).await
}
///Run reverse
pub async fn set_optics_focus_runreverse(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"optics.focus.runreverse","id":"optics.focus.runreverse","params":{}});

    session.call(payload).await
}
///Run reverse for X milliseconds
pub async fn set_optics_focus_runreversetime(
    session: &mut PulseSession,
    milliseconds: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "optics.focus.runreversetime",
        "id": "optics.focus.runreversetime",
        "params": {"milliseconds": milliseconds,}
    });

    session.call(payload).await
}
///Step forward
pub async fn set_optics_focus_stepforward(
    session: &mut PulseSession,
    steps: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "optics.focus.stepforward",
        "id": "optics.focus.stepforward",
        "params": {"steps": steps,}
    });

    session.call(payload).await
}
///Step reverse
pub async fn set_optics_focus_stepreverse(
    session: &mut PulseSession,
    steps: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "optics.focus.stepreverse",
        "id": "optics.focus.stepreverse",
        "params": {"steps": steps,}
    });

    session.call(payload).await
}
///Stop
pub async fn set_optics_focus_stop(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"optics.focus.stop","id":"optics.focus.stop","params":{}});

    session.call(payload).await
}
///List available lenses
pub async fn set_optics_lens_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"optics.lens.list","id":"optics.lens.list","params":{}});

    session.call(payload).await
}
///Calibrate motor
pub async fn set_optics_lensshift_horizontal_calibrate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"optics.lensshift.horizontal.calibrate","id":"optics.lensshift.horizontal.calibrate","params":{}});

    session.call(payload).await
}
///Run forward
pub async fn set_optics_lensshift_horizontal_runforward(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"optics.lensshift.horizontal.runforward","id":"optics.lensshift.horizontal.runforward","params":{}});

    session.call(payload).await
}
///Run forward for X milliseconds
pub async fn set_optics_lensshift_horizontal_runforwardtime(
    session: &mut PulseSession,
    milliseconds: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "optics.lensshift.horizontal.runforwardtime",
        "id": "optics.lensshift.horizontal.runforwardtime",
        "params": {"milliseconds": milliseconds,}
    });

    session.call(payload).await
}
///Run reverse
pub async fn set_optics_lensshift_horizontal_runreverse(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"optics.lensshift.horizontal.runreverse","id":"optics.lensshift.horizontal.runreverse","params":{}});

    session.call(payload).await
}
///Run reverse for X milliseconds
pub async fn set_optics_lensshift_horizontal_runreversetime(
    session: &mut PulseSession,
    milliseconds: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "optics.lensshift.horizontal.runreversetime",
        "id": "optics.lensshift.horizontal.runreversetime",
        "params": {"milliseconds": milliseconds,}
    });

    session.call(payload).await
}
///Step forward
pub async fn set_optics_lensshift_horizontal_stepforward(
    session: &mut PulseSession,
    steps: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "optics.lensshift.horizontal.stepforward",
        "id": "optics.lensshift.horizontal.stepforward",
        "params": {"steps": steps,}
    });

    session.call(payload).await
}
///Step reverse
pub async fn set_optics_lensshift_horizontal_stepreverse(
    session: &mut PulseSession,
    steps: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "optics.lensshift.horizontal.stepreverse",
        "id": "optics.lensshift.horizontal.stepreverse",
        "params": {"steps": steps,}
    });

    session.call(payload).await
}
///Stop
pub async fn set_optics_lensshift_horizontal_stop(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"optics.lensshift.horizontal.stop","id":"optics.lensshift.horizontal.stop","params":{}});

    session.call(payload).await
}
///Calibrate motor
pub async fn set_optics_lensshift_vertical_calibrate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"optics.lensshift.vertical.calibrate","id":"optics.lensshift.vertical.calibrate","params":{}});

    session.call(payload).await
}
///Run forward
pub async fn set_optics_lensshift_vertical_runforward(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"optics.lensshift.vertical.runforward","id":"optics.lensshift.vertical.runforward","params":{}});

    session.call(payload).await
}
///Run forward for X milliseconds
pub async fn set_optics_lensshift_vertical_runforwardtime(
    session: &mut PulseSession,
    milliseconds: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "optics.lensshift.vertical.runforwardtime",
        "id": "optics.lensshift.vertical.runforwardtime",
        "params": {"milliseconds": milliseconds,}
    });

    session.call(payload).await
}
///Run reverse
pub async fn set_optics_lensshift_vertical_runreverse(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"optics.lensshift.vertical.runreverse","id":"optics.lensshift.vertical.runreverse","params":{}});

    session.call(payload).await
}
///Run reverse for X milliseconds
pub async fn set_optics_lensshift_vertical_runreversetime(
    session: &mut PulseSession,
    milliseconds: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "optics.lensshift.vertical.runreversetime",
        "id": "optics.lensshift.vertical.runreversetime",
        "params": {"milliseconds": milliseconds,}
    });

    session.call(payload).await
}
///Step forward
pub async fn set_optics_lensshift_vertical_stepforward(
    session: &mut PulseSession,
    steps: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "optics.lensshift.vertical.stepforward",
        "id": "optics.lensshift.vertical.stepforward",
        "params": {"steps": steps,}
    });

    session.call(payload).await
}
///Step reverse
pub async fn set_optics_lensshift_vertical_stepreverse(
    session: &mut PulseSession,
    steps: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "optics.lensshift.vertical.stepreverse",
        "id": "optics.lensshift.vertical.stepreverse",
        "params": {"steps": steps,}
    });

    session.call(payload).await
}
///Stop
pub async fn set_optics_lensshift_vertical_stop(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"optics.lensshift.vertical.stop","id":"optics.lensshift.vertical.stop","params":{}});

    session.call(payload).await
}
///Shift lens to center of allowed shift range
pub async fn set_optics_shifttocenter(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"optics.shifttocenter","id":"optics.shifttocenter","params":{}});

    session.call(payload).await
}
///Get object path of shutter
pub async fn set_optics_shutter_getobjectpath(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"optics.shutter.getobjectpath","id":"optics.shutter.getobjectpath","params":{}});

    session.call(payload).await
}
///Get state of shutter
pub async fn set_optics_shutter_getstate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"optics.shutter.getstate","id":"optics.shutter.getstate","params":{}});

    session.call(payload).await
}
///Toggle shutter position
pub async fn set_optics_shutter_toggle(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"optics.shutter.toggle","id":"optics.shutter.toggle","params":{}});

    session.call(payload).await
}
///Calibrate motor
pub async fn set_optics_zoom_calibrate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"optics.zoom.calibrate","id":"optics.zoom.calibrate","params":{}});

    session.call(payload).await
}
///Run forward
pub async fn set_optics_zoom_runforward(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"optics.zoom.runforward","id":"optics.zoom.runforward","params":{}});

    session.call(payload).await
}
///Run forward for X milliseconds
pub async fn set_optics_zoom_runforwardtime(
    session: &mut PulseSession,
    milliseconds: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "optics.zoom.runforwardtime",
        "id": "optics.zoom.runforwardtime",
        "params": {"milliseconds": milliseconds,}
    });

    session.call(payload).await
}
///Run reverse
pub async fn set_optics_zoom_runreverse(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"optics.zoom.runreverse","id":"optics.zoom.runreverse","params":{}});

    session.call(payload).await
}
///Run reverse for X milliseconds
pub async fn set_optics_zoom_runreversetime(
    session: &mut PulseSession,
    milliseconds: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "optics.zoom.runreversetime",
        "id": "optics.zoom.runreversetime",
        "params": {"milliseconds": milliseconds,}
    });

    session.call(payload).await
}
///Step forward
pub async fn set_optics_zoom_stepforward(
    session: &mut PulseSession,
    steps: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "optics.zoom.stepforward",
        "id": "optics.zoom.stepforward",
        "params": {"steps": steps,}
    });

    session.call(payload).await
}
///Step reverse
pub async fn set_optics_zoom_stepreverse(
    session: &mut PulseSession,
    steps: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "optics.zoom.stepreverse",
        "id": "optics.zoom.stepreverse",
        "params": {"steps": steps,}
    });

    session.call(payload).await
}
///Stop
pub async fn set_optics_zoom_stop(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"optics.zoom.stop","id":"optics.zoom.stop","params":{}});

    session.call(payload).await
}
///Trigger auto detection of connected peripherals.
pub async fn set_peripheral_autodetect(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"peripheral.autodetect","id":"peripheral.autodetect","params":{}});

    session.call(payload).await
}
///List alignment tools.
pub async fn set_peripheral_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"peripheral.list","id":"peripheral.list","params":{}});

    session.call(payload).await
}
///Activate the profile associated with the given preset.
pub async fn set_profile_activatepreset(
    session: &mut PulseSession,
    preset: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "profile.activatepreset",
        "id": "profile.activatepreset",
        "params": {"preset": preset,}
    });

    session.call(payload).await
}
///Activate the given profile
pub async fn set_profile_activateprofile(
    session: &mut PulseSession,
    name: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "profile.activateprofile",
        "id": "profile.activateprofile",
        "params": {"name": name,}
    });

    session.call(payload).await
}
///Assign a profile to a preset.
pub async fn set_profile_assignpreset(
    session: &mut PulseSession,
    profile: String,preset: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "profile.assignpreset",
        "id": "profile.assignpreset",
        "params": {"profile": profile,"preset": preset,}
    });

    session.call(payload).await
}
///Create a profile containing the current values of the listed domains.
pub async fn set_profile_createprofile(
    session: &mut PulseSession,
    domains: std::vec::Vec<String>,profilename: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "profile.createprofile",
        "id": "profile.createprofile",
        "params": {"domains": domains,"profilename": profilename,}
    });

    session.call(payload).await
}
///Delete the given profile
pub async fn set_profile_deleteprofile(
    session: &mut PulseSession,
    name: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "profile.deleteprofile",
        "id": "profile.deleteprofile",
        "params": {"name": name,}
    });

    session.call(payload).await
}
///Get the list of domains stored in a profile.
pub async fn set_profile_getdomainsforprofile(
    session: &mut PulseSession,
    name: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "profile.getdomainsforprofile",
        "id": "profile.getdomainsforprofile",
        "params": {"name": name,}
    });

    session.call(payload).await
}
///Get the preset assignment for given profile. -1 means unassigned.
pub async fn set_profile_presetforprofile(
    session: &mut PulseSession,
    name: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "profile.presetforprofile",
        "id": "profile.presetforprofile",
        "params": {"name": name,}
    });

    session.call(payload).await
}
///Get the profile associated with the given preset assignment. Empty string means unassigned.
pub async fn set_profile_profileforpreset(
    session: &mut PulseSession,
    preset: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "profile.profileforpreset",
        "id": "profile.profileforpreset",
        "params": {"preset": preset,}
    });

    session.call(payload).await
}
///Return a list of all the object names of the IR sensors
pub async fn set_remotecontrol_listsensors(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"remotecontrol.listsensors","id":"remotecontrol.listsensors","params":{}});

    session.call(payload).await
}
///Create a new action for scheduler.
pub async fn set_scheduler_create(
    session: &mut PulseSession,
    name: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "scheduler.create",
        "id": "scheduler.create",
        "params": {"name": name,}
    });

    session.call(payload).await
}
///Name of the counter
pub async fn set_statistics_illuminationonuptime_getname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.illumination on uptime.getname","id":"statistics.illumination on uptime.getname","params":{}});

    session.call(payload).await
}
///Unit of measurements
pub async fn set_statistics_illuminationonuptime_getunit(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.illumination on uptime.getunit","id":"statistics.illumination on uptime.getunit","params":{}});

    session.call(payload).await
}
///Name of the counter
pub async fn set_statistics_laserruntime_getname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.laser runtime.getname","id":"statistics.laser runtime.getname","params":{}});

    session.call(payload).await
}
///Unit of measurements
pub async fn set_statistics_laserruntime_getunit(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.laser runtime.getunit","id":"statistics.laser runtime.getunit","params":{}});

    session.call(payload).await
}
///Name of the counter
pub async fn set_statistics_laser_plate01_bank01_runtimeseconds_getname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.laser.plate01.bank01.runtimeseconds.getname","id":"statistics.laser.plate01.bank01.runtimeseconds.getname","params":{}});

    session.call(payload).await
}
///Unit of measurements
pub async fn set_statistics_laser_plate01_bank01_runtimeseconds_getunit(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.laser.plate01.bank01.runtimeseconds.getunit","id":"statistics.laser.plate01.bank01.runtimeseconds.getunit","params":{}});

    session.call(payload).await
}
///Name of the counter
pub async fn set_statistics_laser_plate01_bank02_runtimeseconds_getname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.laser.plate01.bank02.runtimeseconds.getname","id":"statistics.laser.plate01.bank02.runtimeseconds.getname","params":{}});

    session.call(payload).await
}
///Unit of measurements
pub async fn set_statistics_laser_plate01_bank02_runtimeseconds_getunit(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.laser.plate01.bank02.runtimeseconds.getunit","id":"statistics.laser.plate01.bank02.runtimeseconds.getunit","params":{}});

    session.call(payload).await
}
///Name of the counter
pub async fn set_statistics_laser_plate02_bank01_runtimeseconds_getname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.laser.plate02.bank01.runtimeseconds.getname","id":"statistics.laser.plate02.bank01.runtimeseconds.getname","params":{}});

    session.call(payload).await
}
///Unit of measurements
pub async fn set_statistics_laser_plate02_bank01_runtimeseconds_getunit(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.laser.plate02.bank01.runtimeseconds.getunit","id":"statistics.laser.plate02.bank01.runtimeseconds.getunit","params":{}});

    session.call(payload).await
}
///Name of the counter
pub async fn set_statistics_laser_plate02_bank02_runtimeseconds_getname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.laser.plate02.bank02.runtimeseconds.getname","id":"statistics.laser.plate02.bank02.runtimeseconds.getname","params":{}});

    session.call(payload).await
}
///Unit of measurements
pub async fn set_statistics_laser_plate02_bank02_runtimeseconds_getunit(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.laser.plate02.bank02.runtimeseconds.getunit","id":"statistics.laser.plate02.bank02.runtimeseconds.getunit","params":{}});

    session.call(payload).await
}
///List all counter names
pub async fn set_statistics_listcounters(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.listcounters","id":"statistics.listcounters","params":{}});

    session.call(payload).await
}
///Name of the counter
pub async fn set_statistics_maintenance_actuator_getname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.maintenance.actuator.getname","id":"statistics.maintenance.actuator.getname","params":{}});

    session.call(payload).await
}
///Unit of measurements
pub async fn set_statistics_maintenance_actuator_getunit(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.maintenance.actuator.getunit","id":"statistics.maintenance.actuator.getunit","params":{}});

    session.call(payload).await
}
///Name of the counter
pub async fn set_statistics_maintenance_colorwheel_getname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.maintenance.colorwheel.getname","id":"statistics.maintenance.colorwheel.getname","params":{}});

    session.call(payload).await
}
///Unit of measurements
pub async fn set_statistics_maintenance_colorwheel_getunit(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.maintenance.colorwheel.getunit","id":"statistics.maintenance.colorwheel.getunit","params":{}});

    session.call(payload).await
}
///Name of the counter
pub async fn set_statistics_maintenance_phosphorwheel_getname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.maintenance.phosphorwheel.getname","id":"statistics.maintenance.phosphorwheel.getname","params":{}});

    session.call(payload).await
}
///Unit of measurements
pub async fn set_statistics_maintenance_phosphorwheel_getunit(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.maintenance.phosphorwheel.getunit","id":"statistics.maintenance.phosphorwheel.getunit","params":{}});

    session.call(payload).await
}
///Name of the counter
pub async fn set_statistics_projectoron_getname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.projector on.getname","id":"statistics.projector on.getname","params":{}});

    session.call(payload).await
}
///Unit of measurements
pub async fn set_statistics_projectoron_getunit(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.projector on.getunit","id":"statistics.projector on.getunit","params":{}});

    session.call(payload).await
}
///Name of the counter
pub async fn set_statistics_projectoruptime_getname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.projector uptime.getname","id":"statistics.projector uptime.getname","params":{}});

    session.call(payload).await
}
///Unit of measurements
pub async fn set_statistics_projectoruptime_getunit(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.projector uptime.getunit","id":"statistics.projector uptime.getunit","params":{}});

    session.call(payload).await
}
///Name of the counter
pub async fn set_statistics_projector_getname(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.projector.getname","id":"statistics.projector.getname","params":{}});

    session.call(payload).await
}
///Unit of measurements
pub async fn set_statistics_projector_getunit(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"statistics.projector.getunit","id":"statistics.projector.getunit","params":{}});

    session.call(payload).await
}
///Signal user activity (resets timeout countdown timers)
pub async fn set_system_activity(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"system.activity","id":"system.activity","params":{}});

    session.call(payload).await
}
///Retrieve list of detected boards
pub async fn set_system_boards_getboardlist(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"system.boards.getboardlist","id":"system.boards.getboardlist","params":{}});

    session.call(payload).await
}
///Retrieve list of missing boards
pub async fn set_system_boards_getmissingboardlist(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"system.boards.getmissingboardlist","id":"system.boards.getmissingboardlist","params":{}});

    session.call(payload).await
}
///Returns the system date as local time.
pub async fn set_system_date_getlocaldate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"system.date.getlocaldate","id":"system.date.getlocaldate","params":{}});

    session.call(payload).await
}
///Returns the system date as UTC time.
pub async fn set_system_date_getsystemdate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"system.date.getsystemdate","id":"system.date.getsystemdate","params":{}});

    session.call(payload).await
}
///Description not provided
pub async fn set_system_getidentifications(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"system.getidentifications","id":"system.getidentifications","params":{}});

    session.call(payload).await
}
///Description not provided
pub async fn set_system_getsystemdate(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"system.getsystemdate","id":"system.getsystemdate","params":{}});

    session.call(payload).await
}
///Set the device in the eco state
pub async fn set_system_gotoeco(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"system.gotoeco","id":"system.gotoeco","params":{}});

    session.call(payload).await
}
///Set the device from standby state to ready state
pub async fn set_system_gotoready(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"system.gotoready","id":"system.gotoready","params":{}});

    session.call(payload).await
}
///Save registrationfile to usb memory device
pub async fn set_system_license_file_saveregistrationtousb(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"system.license.file.saveregistrationtousb","id":"system.license.file.saveregistrationtousb","params":{}});

    session.call(payload).await
}
///Upload license from usb memory device
pub async fn set_system_license_file_uploadfromusb(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"system.license.file.uploadfromusb","id":"system.license.file.uploadfromusb","params":{}});

    session.call(payload).await
}
///Returns the list of available reset domains
pub async fn set_system_listresetdomains(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"system.listresetdomains","id":"system.listresetdomains","params":{}});

    session.call(payload).await
}
///Power off the unit
pub async fn set_system_poweroff(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"system.poweroff","id":"system.poweroff","params":{}});

    session.call(payload).await
}
///Power on the unit
pub async fn set_system_poweron(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"system.poweron","id":"system.poweron","params":{}});

    session.call(payload).await
}
///
pub async fn set_system_reset(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"system.reset","id":"system.reset","params":{}});

    session.call(payload).await
}
///'Performed'-signals. Subsequent calls to 'ResetAll' or 'Reset' will fail until all domains have completed.
pub async fn set_Asynchronouslystartsresetofselecteddomains_Thecompletionofthedomainsaresignalledbyoneoremore(
    session: &mut PulseSession,
    domains: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "Asynchronously starts reset of selected domains. The completion of the domains are signalled by one ore more",
        "id": "Asynchronously starts reset of selected domains. The completion of the domains are signalled by one ore more",
        "params": {"domains": domains,}
    });

    session.call(payload).await
}
///Asynchronously starts reset of all domains. The completion of the domains are signalled by one ore more 'Performed'- signals. Subsequent calls to 'ResetAll' or 'Reset' will fail until all domains have completed. Execute this function with caution, as it might remove the projector's hostname, requiring physical access to the projector.
pub async fn set_system_resetall(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"system.resetall","id":"system.resetall","params":{}});

    session.call(payload).await
}
///Set the state and color of the key LEDs
pub async fn set_ui_leds_setstate(
    session: &mut PulseSession,
    key: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "ui.leds.setstate",
        "id": "ui.leds.setstate",
        "params": {"key": key,}
    });

    session.call(payload).await
}
///Get the value of the specified key
pub async fn set_ui_settings_get(
    session: &mut PulseSession,
    key: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "ui.settings.get",
        "id": "ui.settings.get",
        "params": {"key": key,}
    });

    session.call(payload).await
}
///
pub async fn set_ui_settings_getfonticons(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"ui.settings.getfonticons","id":"ui.settings.getfonticons","params":{}});

    session.call(payload).await
}
///name and the icon class name.
pub async fn set_Returnadictionaryoficonsforthespecifiedcategory_Theniconisreturnedasastringcontainingthefontsetclass(
    session: &mut PulseSession,
    category: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "Return a dictionary of icons for the specified category. Then icon is returned as a string containing the font set class",
        "id": "Return a dictionary of icons for the specified category. Then icon is returned as a string containing the font set class",
        "params": {"category": category,}
    });

    session.call(payload).await
}
///Return a dictionary of icons for the specified category. The icon is return as a SVG sprite name.
pub async fn set_ui_settings_geticons(
    session: &mut PulseSession,
    category: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "ui.settings.geticons",
        "id": "ui.settings.geticons",
        "params": {"category": category,}
    });

    session.call(payload).await
}
///Return a list of all the keys
pub async fn set_ui_settings_keys(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"ui.settings.keys","id":"ui.settings.keys","params":{}});

    session.call(payload).await
}
///Return a list of key/value pairs of all the settings
pub async fn set_ui_settings_list(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"ui.settings.list","id":"ui.settings.list","params":{}});

    session.call(payload).await
}
///Remove the specfied key and value
pub async fn set_ui_settings_remove(
    session: &mut PulseSession,
    key: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "ui.settings.remove",
        "id": "ui.settings.remove",
        "params": {"key": key,}
    });

    session.call(payload).await
}
///Set the key to the specified value
pub async fn set_ui_settings_set(
    session: &mut PulseSession,
    key: String,value: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "ui.settings.set",
        "id": "ui.settings.set",
        "params": {"key": key,"value": value,}
    });

    session.call(payload).await
}
///Toggle lens menu on or off
pub async fn set_ui_togglelensmenu(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"ui.togglelensmenu","id":"ui.togglelensmenu","params":{}});

    session.call(payload).await
}
///Toggle menu on/off
pub async fn set_ui_togglemenu(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"ui.togglemenu","id":"ui.togglemenu","params":{}});

    session.call(payload).await
}
///Toggle on screen display on or off
pub async fn set_ui_toggleosd(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"ui.toggleosd","id":"ui.toggleosd","params":{}});

    session.call(payload).await
}
///Toggle the pattern menu on or off
pub async fn set_ui_togglepatternmenu(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"ui.togglepatternmenu","id":"ui.togglepatternmenu","params":{}});

    session.call(payload).await
}
///Toggle the source menu on or off
pub async fn set_ui_togglesourcemenu(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"ui.togglesourcemenu","id":"ui.togglesourcemenu","params":{}});

    session.call(payload).await
}
///[DEPRECATED]
pub async fn set_ui_togglestealthmode(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"ui.togglestealthmode","id":"ui.togglestealthmode","params":{}});

    session.call(payload).await
}
///Called from touchpanel when the touchpanel is in Test mode
pub async fn set_ui_touchscreen_tapped(
    session: &mut PulseSession,
    x: std::collections::HashMap<String, String>,y: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "ui.touchscreen.tapped",
        "id": "ui.touchscreen.tapped",
        "params": {"x": x,"y": y,}
    });

    session.call(payload).await
}
///Change the user group
pub async fn set_user_admin_changegroup(
    session: &mut PulseSession,
    newgroup: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.admin.changegroup",
        "id": "user.admin.changegroup",
        "params": {"newgroup": newgroup,}
    });

    session.call(payload).await
}
///Change the password
pub async fn set_user_admin_changepassword(
    session: &mut PulseSession,
    newpassword: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.admin.changepassword",
        "id": "user.admin.changepassword",
        "params": {"newpassword": newpassword,}
    });

    session.call(payload).await
}
///Change the 6-digit PIN code
pub async fn set_user_admin_changepincode(
    session: &mut PulseSession,
    newpincode: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.admin.changepincode",
        "id": "user.admin.changepincode",
        "params": {"newpincode": newpincode,}
    });

    session.call(payload).await
}
///Change the public key
pub async fn set_user_admin_changepublickey(
    session: &mut PulseSession,
    newpublickey: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.admin.changepublickey",
        "id": "user.admin.changepublickey",
        "params": {"newpublickey": newpublickey,}
    });

    session.call(payload).await
}
///Change the username
pub async fn set_user_admin_changeusername(
    session: &mut PulseSession,
    newusername: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.admin.changeusername",
        "id": "user.admin.changeusername",
        "params": {"newusername": newusername,}
    });

    session.call(payload).await
}
///Disable the user
pub async fn set_user_admin_disable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.admin.disable","id":"user.admin.disable","params":{}});

    session.call(payload).await
}
///Enable the user
pub async fn set_user_admin_enable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.admin.enable","id":"user.admin.enable","params":{}});

    session.call(payload).await
}
///Invalidate all tokens
pub async fn set_user_admin_invalidatetokens(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.admin.invalidatetokens","id":"user.admin.invalidatetokens","params":{}});

    session.call(payload).await
}
///Invalidate all tokens and log out of all active sessions
pub async fn set_user_admin_logouteverywhere(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.admin.logouteverywhere","id":"user.admin.logouteverywhere","params":{}});

    session.call(payload).await
}
///Remove the password
pub async fn set_user_admin_removepassword(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.admin.removepassword","id":"user.admin.removepassword","params":{}});

    session.call(payload).await
}
///Remove the 6-digit PIN code
pub async fn set_user_admin_removepincode(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.admin.removepincode","id":"user.admin.removepincode","params":{}});

    session.call(payload).await
}
///Change the user group
pub async fn set_user_currentuser_changegroup(
    session: &mut PulseSession,
    newgroup: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.currentuser.changegroup",
        "id": "user.currentuser.changegroup",
        "params": {"newgroup": newgroup,}
    });

    session.call(payload).await
}
///Change the password
pub async fn set_user_currentuser_changepassword(
    session: &mut PulseSession,
    newpassword: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.currentuser.changepassword",
        "id": "user.currentuser.changepassword",
        "params": {"newpassword": newpassword,}
    });

    session.call(payload).await
}
///Change the 6-digit PIN code
pub async fn set_user_currentuser_changepincode(
    session: &mut PulseSession,
    newpincode: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.currentuser.changepincode",
        "id": "user.currentuser.changepincode",
        "params": {"newpincode": newpincode,}
    });

    session.call(payload).await
}
///Change the public key
pub async fn set_user_currentuser_changepublickey(
    session: &mut PulseSession,
    newpublickey: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.currentuser.changepublickey",
        "id": "user.currentuser.changepublickey",
        "params": {"newpublickey": newpublickey,}
    });

    session.call(payload).await
}
///Change the username
pub async fn set_user_currentuser_changeusername(
    session: &mut PulseSession,
    newusername: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.currentuser.changeusername",
        "id": "user.currentuser.changeusername",
        "params": {"newusername": newusername,}
    });

    session.call(payload).await
}
///Disable the user
pub async fn set_user_currentuser_disable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.currentuser.disable","id":"user.currentuser.disable","params":{}});

    session.call(payload).await
}
///Enable the user
pub async fn set_user_currentuser_enable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.currentuser.enable","id":"user.currentuser.enable","params":{}});

    session.call(payload).await
}
///Invalidate all tokens
pub async fn set_user_currentuser_invalidatetokens(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.currentuser.invalidatetokens","id":"user.currentuser.invalidatetokens","params":{}});

    session.call(payload).await
}
///Invalidate all tokens and log out of all active sessions
pub async fn set_user_currentuser_logouteverywhere(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.currentuser.logouteverywhere","id":"user.currentuser.logouteverywhere","params":{}});

    session.call(payload).await
}
///Remove the password
pub async fn set_user_currentuser_removepassword(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.currentuser.removepassword","id":"user.currentuser.removepassword","params":{}});

    session.call(payload).await
}
///Remove the 6-digit PIN code
pub async fn set_user_currentuser_removepincode(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.currentuser.removepincode","id":"user.currentuser.removepincode","params":{}});

    session.call(payload).await
}
///Change the user group
pub async fn set_user_poweruser_changegroup(
    session: &mut PulseSession,
    newgroup: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.poweruser.changegroup",
        "id": "user.poweruser.changegroup",
        "params": {"newgroup": newgroup,}
    });

    session.call(payload).await
}
///Change the password
pub async fn set_user_poweruser_changepassword(
    session: &mut PulseSession,
    newpassword: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.poweruser.changepassword",
        "id": "user.poweruser.changepassword",
        "params": {"newpassword": newpassword,}
    });

    session.call(payload).await
}
///Change the 6-digit PIN code
pub async fn set_user_poweruser_changepincode(
    session: &mut PulseSession,
    newpincode: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.poweruser.changepincode",
        "id": "user.poweruser.changepincode",
        "params": {"newpincode": newpincode,}
    });

    session.call(payload).await
}
///Change the public key
pub async fn set_user_poweruser_changepublickey(
    session: &mut PulseSession,
    newpublickey: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.poweruser.changepublickey",
        "id": "user.poweruser.changepublickey",
        "params": {"newpublickey": newpublickey,}
    });

    session.call(payload).await
}
///Change the username
pub async fn set_user_poweruser_changeusername(
    session: &mut PulseSession,
    newusername: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.poweruser.changeusername",
        "id": "user.poweruser.changeusername",
        "params": {"newusername": newusername,}
    });

    session.call(payload).await
}
///Disable the user
pub async fn set_user_poweruser_disable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.poweruser.disable","id":"user.poweruser.disable","params":{}});

    session.call(payload).await
}
///Enable the user
pub async fn set_user_poweruser_enable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.poweruser.enable","id":"user.poweruser.enable","params":{}});

    session.call(payload).await
}
///Invalidate all tokens
pub async fn set_user_poweruser_invalidatetokens(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.poweruser.invalidatetokens","id":"user.poweruser.invalidatetokens","params":{}});

    session.call(payload).await
}
///Invalidate all tokens and log out of all active sessions
pub async fn set_user_poweruser_logouteverywhere(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.poweruser.logouteverywhere","id":"user.poweruser.logouteverywhere","params":{}});

    session.call(payload).await
}
///Remove the password
pub async fn set_user_poweruser_removepassword(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.poweruser.removepassword","id":"user.poweruser.removepassword","params":{}});

    session.call(payload).await
}
///Remove the 6-digit PIN code
pub async fn set_user_poweruser_removepincode(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.poweruser.removepincode","id":"user.poweruser.removepincode","params":{}});

    session.call(payload).await
}
///Reset the administrator account using the response provided by barco
pub async fn set_user_resetadministrator(
    session: &mut PulseSession,
    response: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.resetadministrator",
        "id": "user.resetadministrator",
        "params": {"response": response,}
    });

    session.call(payload).await
}
///Change the user group
pub async fn set_user_testuser_changegroup(
    session: &mut PulseSession,
    newgroup: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.testuser.changegroup",
        "id": "user.testuser.changegroup",
        "params": {"newgroup": newgroup,}
    });

    session.call(payload).await
}
///Change the password
pub async fn set_user_testuser_changepassword(
    session: &mut PulseSession,
    newpassword: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.testuser.changepassword",
        "id": "user.testuser.changepassword",
        "params": {"newpassword": newpassword,}
    });

    session.call(payload).await
}
///Change the 6-digit PIN code
pub async fn set_user_testuser_changepincode(
    session: &mut PulseSession,
    newpincode: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.testuser.changepincode",
        "id": "user.testuser.changepincode",
        "params": {"newpincode": newpincode,}
    });

    session.call(payload).await
}
///Change the public key
pub async fn set_user_testuser_changepublickey(
    session: &mut PulseSession,
    newpublickey: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.testuser.changepublickey",
        "id": "user.testuser.changepublickey",
        "params": {"newpublickey": newpublickey,}
    });

    session.call(payload).await
}
///Change the username
pub async fn set_user_testuser_changeusername(
    session: &mut PulseSession,
    newusername: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.testuser.changeusername",
        "id": "user.testuser.changeusername",
        "params": {"newusername": newusername,}
    });

    session.call(payload).await
}
///Disable the user
pub async fn set_user_testuser_disable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.testuser.disable","id":"user.testuser.disable","params":{}});

    session.call(payload).await
}
///Enable the user
pub async fn set_user_testuser_enable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.testuser.enable","id":"user.testuser.enable","params":{}});

    session.call(payload).await
}
///Invalidate all tokens
pub async fn set_user_testuser_invalidatetokens(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.testuser.invalidatetokens","id":"user.testuser.invalidatetokens","params":{}});

    session.call(payload).await
}
///Invalidate all tokens and log out of all active sessions
pub async fn set_user_testuser_logouteverywhere(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.testuser.logouteverywhere","id":"user.testuser.logouteverywhere","params":{}});

    session.call(payload).await
}
///Remove the password
pub async fn set_user_testuser_removepassword(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.testuser.removepassword","id":"user.testuser.removepassword","params":{}});

    session.call(payload).await
}
///Remove the 6-digit PIN code
pub async fn set_user_testuser_removepincode(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.testuser.removepincode","id":"user.testuser.removepincode","params":{}});

    session.call(payload).await
}
///Change the user group
pub async fn set_user_user_changegroup(
    session: &mut PulseSession,
    newgroup: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.user.changegroup",
        "id": "user.user.changegroup",
        "params": {"newgroup": newgroup,}
    });

    session.call(payload).await
}
///Change the password
pub async fn set_user_user_changepassword(
    session: &mut PulseSession,
    newpassword: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.user.changepassword",
        "id": "user.user.changepassword",
        "params": {"newpassword": newpassword,}
    });

    session.call(payload).await
}
///Change the 6-digit PIN code
pub async fn set_user_user_changepincode(
    session: &mut PulseSession,
    newpincode: std::collections::HashMap<String, String>,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.user.changepincode",
        "id": "user.user.changepincode",
        "params": {"newpincode": newpincode,}
    });

    session.call(payload).await
}
///Change the public key
pub async fn set_user_user_changepublickey(
    session: &mut PulseSession,
    newpublickey: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.user.changepublickey",
        "id": "user.user.changepublickey",
        "params": {"newpublickey": newpublickey,}
    });

    session.call(payload).await
}
///Change the username
pub async fn set_user_user_changeusername(
    session: &mut PulseSession,
    newusername: String,) -> APICallResult {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "user.user.changeusername",
        "id": "user.user.changeusername",
        "params": {"newusername": newusername,}
    });

    session.call(payload).await
}
///Disable the user
pub async fn set_user_user_disable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.user.disable","id":"user.user.disable","params":{}});

    session.call(payload).await
}
///Enable the user
pub async fn set_user_user_enable(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.user.enable","id":"user.user.enable","params":{}});

    session.call(payload).await
}
///Invalidate all tokens
pub async fn set_user_user_invalidatetokens(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.user.invalidatetokens","id":"user.user.invalidatetokens","params":{}});

    session.call(payload).await
}
///Invalidate all tokens and log out of all active sessions
pub async fn set_user_user_logouteverywhere(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.user.logouteverywhere","id":"user.user.logouteverywhere","params":{}});

    session.call(payload).await
}
///Remove the password
pub async fn set_user_user_removepassword(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.user.removepassword","id":"user.user.removepassword","params":{}});

    session.call(payload).await
}
///Remove the 6-digit PIN code
pub async fn set_user_user_removepincode(session: &mut PulseSession) -> APICallResult {
    let payload = serde_json::json!({"jsonrpc":"2.0","method":"user.user.removepincode","id":"user.user.removepincode","params":{}});

    session.call(payload).await
}
