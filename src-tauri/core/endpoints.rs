#[allow(unused_imports)]
use crate::core::transfer::*;

get!("/resource", resources, HueV2ResourceResponse);
get!("/resource/{id}", resource, HueV2ResourceResponse, [id]);
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

handlers!(
    get_resources,
    get_resource,
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
    get_light_level
);
