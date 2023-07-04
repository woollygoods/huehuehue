use std::net::IpAddr;

use tauri::State;

#[allow(unused_imports)]
use crate::core::transfer::*;
use crate::{HueHueHueError, HueHueHueState};

get!("/resource/light", lights, HueV2LightResponse);
get!("/resource/light/{id}", light, HueV2LightResponse, [id]);
get!("/resource/scene", scenes, HueV2SceneResponse);
get!("/resource/scene/{id}", scene, HueV2SceneResponse, [id]);
get!("/resource/room", rooms, HueV2RoomResponse);
get!("/resource/room/{id}", room, HueV2RoomResponse, [id]);
get!("/resource/zone", zones, HueV2ZoneResponse);
get!("/resource/zone/{id}", zone, HueV2ZoneResponse, [id]);
get!(
    "/resource/bridge_home",
    bridge_homes,
    HueV2BridgeHomeResponse
);
get!(
    "/resource/bridge_home/{id}",
    bridge_home,
    HueV2BridgeHomeResponse,
    [id]
);
get!(
    "/resource/grouped_light",
    grouped_lights,
    HueV2GroupedLightResponse
);
get!(
    "/resource/grouped_light/{id}",
    grouped_light,
    HueV2GroupedLightResponse,
    [id]
);
get!("/resource/device", devices, HueV2DeviceResponse);
get!("/resource/device/{id}", device, HueV2DeviceResponse, [id]);
get!("/resource/bridge", bridges, HueV2BridgeResponse);
get!("/resource/bridge/{id}", bridge, HueV2BridgeResponse, [id]);
get!(
    "/resource/device_power",
    device_powers,
    HueV2DevicePowerResponse
);
get!(
    "/resource/device_power/{id}",
    device_power,
    HueV2DevicePowerResponse,
    [id]
);
get!(
    "/resource/zigbee_connectivity",
    zigbee_connectivities,
    HueV2ZigbeeConnectivityResponse
);
get!(
    "/resource/zigbee_connectivity/{id}",
    zigbee_connectivity,
    HueV2ZigbeeConnectivityResponse,
    [id]
);
get!(
    "/resource/zgp_connectivity",
    zgp_connectivities,
    HueV2ZgpConnectivityResponse
);
get!(
    "/resource/zgp_connectivity/{id}",
    zgp_connectivity,
    HueV2ZgpConnectivityResponse,
    [id]
);
get!(
    "/resource/zigbee_device_discovery",
    zigbee_device_discovery,
    HueV2ZigbeeDeviceDiscoveryResponse
);
get!(
    "/resource/zigbee_device_discovery/{id}",
    zigbee_device_discoveries,
    HueV2ZigbeeDeviceDiscoveryResponse,
    [id]
);
get!("/resource/motion", motions, HueV2MotionResponse);
get!("/resource/motion/{id}", motion, HueV2MotionResponse, [id]);
get!(
    "/resource/temperature",
    temperatures,
    HueV2TemperatureResponse
);
get!(
    "/resource/temperature/{id}",
    temperature,
    HueV2TemperatureResponse,
    [id]
);
get!(
    "/resource/light_level",
    light_levels,
    HueV2LightLevelResponse
);
get!(
    "/resource/light_level/{id}",
    light_level,
    HueV2LightLevelResponse,
    [id]
);
get!("/resource/button", buttons, HueV2ButtonResponse);
get!("/resource/button/{id}", button, HueV2ButtonResponse, [id]);
get!(
    "/resource/relative_rotary",
    relative_rotaries,
    HueV2RelativeRotaryResponse
);
get!(
    "/resource/relative_rotary/{id}",
    relative_rotary,
    HueV2RelativeRotaryResponse,
    [id]
);
get!(
    "/resource/behaviour_script",
    behaviour_scripts,
    HueV2BehaviorScriptResponse
);
get!(
    "/resource/behaviour_script/{id}",
    behaviour_script,
    HueV2BehaviorScriptResponse,
    [id]
);
get!(
    "/resource/behaviour_instance",
    behaviour_instances,
    HueV2BehaviorInstanceResponse
);
get!(
    "/resource/behaviour_instance/{id}",
    behaviour_instance,
    HueV2BehaviorInstanceResponse,
    [id]
);
get!(
    "/resource/geofence_client",
    geofence_clients,
    HueV2GeofenceClientResponse
);
get!(
    "/resource/behaviour_instance/{id}",
    geofence_client,
    HueV2GeofenceClientResponse,
    [id]
);
get!(
    "/resource/geolocation",
    geolocations,
    HueV2GeolocationResponse
);
get!(
    "/resource/geolocation/{id}",
    geolocation,
    HueV2GeolocationResponse,
    [id]
);
get!(
    "/resource/entertainment_configuration",
    entertainment_configurations,
    HueV2EntertainmentConfigurationResponse
);
get!(
    "/resource/entertainment_configuration/{id}",
    entertainment_configuration,
    HueV2EntertainmentConfigurationResponse,
    [id]
);
get!(
    "/resource/entertainment",
    entertainments,
    HueV2EntertainmentResponse
);
get!(
    "/resource/entertainment/{id}",
    entertainment,
    HueV2EntertainmentResponse,
    [id]
);
get!("/resource/homekit", homekits, HueV2HomekitResponse);
get!(
    "/resource/homekit/{id}",
    homekit,
    HueV2HomekitResponse,
    [id]
);
get!("/resource/matter", matters, HueV2MatterResponse);
get!("/resource/matter/{id}", matter, HueV2MatterResponse, [id]);
get!(
    "/resource/matter_fabric",
    matter_fabrics,
    HueV2MatterFabricResponse
);
get!(
    "/resource/matter_fabric/{id}",
    matter_fabric,
    HueV2MatterFabricResponse,
    [id]
);
get!("/resource", resources, HueV2ResourceResponse);
get!("/resource/{id}", resource, HueV2ResourceResponse, [id]);
get!(
    "/resource/smart_scene",
    smart_scenes,
    HueV2SmartSceneResponse
);
get!(
    "/resource/smart_scene/{id}",
    smart_scene,
    HueV2SmartSceneResponse,
    [id]
);

#[tauri::command]
#[specta::specta]
pub async fn set_selected_bridge(
    mdns_name: String,
    state: State<'_, HueHueHueState>,
) -> Result<(), HueHueHueError> {
    let mut huehuehue = state.0.lock().await;
    huehuehue.set_selected_bridge(mdns_name);

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_discovered_bridges(
    state: State<'_, HueHueHueState>,
) -> Result<Vec<(String, IpAddr)>, HueHueHueError> {
    let huehuehue = state.0.lock().await;

    Ok(huehuehue
        .get_discovered_bridges()
        .await
        .iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect())
}

handlers!(
    get_lights,
    get_light,
    get_scenes,
    get_scene,
    get_rooms,
    get_room,
    get_zones,
    get_zone,
    get_bridge_homes,
    get_bridge_home,
    get_grouped_lights,
    get_grouped_light,
    get_devices,
    get_device,
    get_bridges,
    get_bridge,
    get_device_powers,
    get_device_power,
    get_zigbee_connectivities,
    get_zigbee_connectivity,
    get_zgp_connectivities,
    get_zgp_connectivity,
    get_zigbee_device_discoveries,
    get_zigbee_device_discovery,
    get_temperatures,
    get_temperature,
    get_light_levels,
    get_light_level,
    get_buttons,
    get_button,
    get_relative_rotaries,
    get_relative_rotary,
    get_behaviour_scripts,
    get_behaviour_script,
    get_behaviour_instances,
    get_behaviour_instance,
    get_geofence_clients,
    get_geofence_client,
    get_geolocations,
    get_geolocation,
    get_entertainment_configurations,
    get_entertainment_configuration,
    get_entertainments,
    get_entertainment,
    get_homekits,
    get_homekit,
    get_matters,
    get_matter,
    get_matter_fabrics,
    get_matter_fabric,
    get_resources,
    get_resource,
    get_smart_scenes,
    get_smart_scene,
    set_selected_bridge,
    get_discovered_bridges
);
