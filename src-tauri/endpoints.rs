#[allow(unused_imports)]
use crate::transfer::*;

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
    get_device
);
