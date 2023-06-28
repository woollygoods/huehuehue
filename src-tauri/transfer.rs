#![allow(unused)]

use core::fmt::Debug;
use paste::paste;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HueV2Error {
    description: String,
}

pub trait HueV2ResGet: Debug {}

#[derive(Debug, Deserialize)]
pub struct HueV2ResourceApi<F: HueV2ResGet> {
    errors: Vec<HueV2Error>,
    data: Vec<F>,
}

macro_rules! hue_v2_resp {
    ($v:ident) => {
        paste! {
            pub type [<$v Response>] = HueV2ResourceApi<$v>;
        }
    };
}

macro_rules! hue_v2_res_get {
    ($v:ident) => {
        impl HueV2ResGet for $v {}

        hue_v2_resp!($v);
    };
}

#[derive(Debug, Deserialize)]
pub enum HueV2ResourceGetType {
    Device,
    BridgeHome,
    Room,
    Zone,
    Light,
    Button,
    RelativeRotary,
    Temperature,
    LightLevel,
    Motion,
    Entertainment,
    GroupedLight,
    DevicePower,
    ZigbeeBridgeConnectivity,
    ZigbeeConnectivity,
    ZgpConnectivity,
    Bridge,
    ZigbeeDeviceDiscovery,
    Homekit,
    Matter,
    MatterFabric,
    Scene,
    EntertainmentConfiguration,
    PublicImage,
    AuthV1,
    BehaviorScript,
    BehaviorInstance,
    Geofence,
    GeofenceClient,
    Geolocation,
    SmartScene,
}

#[derive(Debug, Deserialize)]
pub struct HueV2ResourceGet {
    id: String,
    r#type: HueV2ResourceGetType,
    id_v1: String,
}
hue_v2_res_get!(HueV2ResourceGet);

#[derive(Debug, Deserialize)]
pub struct HueV2DeviceGet {}
hue_v2_res_get!(HueV2DeviceGet);

#[derive(Debug, Deserialize)]
pub struct HueV2BridgeHomeGet {}
hue_v2_res_get!(HueV2BridgeHomeGet);

#[derive(Debug, Deserialize)]
pub struct HueV2RoomGet {}
hue_v2_res_get!(HueV2RoomGet);

#[derive(Debug, Deserialize)]
pub struct HueV2ZoneGet {}
hue_v2_res_get!(HueV2ZoneGet);

#[derive(Debug, Deserialize)]
pub struct HueV2LightGet {}
hue_v2_res_get!(HueV2LightGet);

#[derive(Debug, Deserialize)]
pub struct HueV2ButtonGet {}
hue_v2_res_get!(HueV2ButtonGet);

#[derive(Debug, Deserialize)]
pub struct HueV2RelativeRotaryGet {}
hue_v2_res_get!(HueV2RelativeRotaryGet);

#[derive(Debug, Deserialize)]
pub struct HueV2TemperatureGet {}
hue_v2_res_get!(HueV2TemperatureGet);

#[derive(Debug, Deserialize)]
pub struct HueV2LightLevelGet {}
hue_v2_res_get!(HueV2LightLevelGet);

#[derive(Debug, Deserialize)]
pub struct HueV2MotionGet {}
hue_v2_res_get!(HueV2MotionGet);

#[derive(Debug, Deserialize)]
pub struct HueV2EntertainmentGet {}
hue_v2_res_get!(HueV2EntertainmentGet);

#[derive(Debug, Deserialize)]
pub struct HueV2GroupedLightGet {}
hue_v2_res_get!(HueV2GroupedLightGet);

#[derive(Debug, Deserialize)]
pub struct HueV2DevicePowerGet {}
hue_v2_res_get!(HueV2DevicePowerGet);

#[derive(Debug, Deserialize)]
pub struct HueV2ZigbeeBridgeConnectivityGet {}
hue_v2_res_get!(HueV2ZigbeeBridgeConnectivityGet);

#[derive(Debug, Deserialize)]
pub struct HueV2ZigbeeConnectivityGet {}
hue_v2_res_get!(HueV2ZigbeeConnectivityGet);

#[derive(Debug, Deserialize)]
pub struct HueV2ZgpConnectivityGet {}
hue_v2_res_get!(HueV2ZgpConnectivityGet);

#[derive(Debug, Deserialize)]
pub struct HueV2BridgeGet {}
hue_v2_res_get!(HueV2BridgeGet);

#[derive(Debug, Deserialize)]
pub struct HueV2ZigbeeDeviceDiscoveryGet {}
hue_v2_res_get!(HueV2ZigbeeDeviceDiscoveryGet);

#[derive(Debug, Deserialize)]
pub struct HueV2HomekitGet {}
hue_v2_res_get!(HueV2HomekitGet);

#[derive(Debug, Deserialize)]
pub struct HueV2MatterGet {}
hue_v2_res_get!(HueV2MatterGet);

#[derive(Debug, Deserialize)]
pub struct HueV2MatterFabricGet {}
hue_v2_res_get!(HueV2MatterFabricGet);

#[derive(Debug, Deserialize)]
pub struct HueV2SceneGet {}
hue_v2_res_get!(HueV2SceneGet);

#[derive(Debug, Deserialize)]
pub struct HueV2EntertainmentConfigurationGet {}
hue_v2_res_get!(HueV2EntertainmentConfigurationGet);

#[derive(Debug, Deserialize)]
pub struct HueV2PublicImageGet {}
hue_v2_res_get!(HueV2PublicImageGet);

#[derive(Debug, Deserialize)]
pub struct HueV2AuthV1Get {}
hue_v2_res_get!(HueV2AuthV1Get);

#[derive(Debug, Deserialize)]
pub struct HueV2BehaviorScriptGet {}
hue_v2_res_get!(HueV2BehaviorScriptGet);

#[derive(Debug, Deserialize)]
pub struct HueV2BehaviorInstanceGet {}
hue_v2_res_get!(HueV2BehaviorInstanceGet);

#[derive(Debug, Deserialize)]
pub struct HueV2GeofenceGet {}
hue_v2_res_get!(HueV2GeofenceGet);

#[derive(Debug, Deserialize)]
pub struct HueV2GeofenceClientGet {}
hue_v2_res_get!(HueV2GeofenceClientGet);

#[derive(Debug, Deserialize)]
pub struct HueV2GeolocationGet {}
hue_v2_res_get!(HueV2GeolocationGet);

#[derive(Debug, Deserialize)]
pub struct HueV2SmartSceneGet {}
hue_v2_res_get!(HueV2SmartSceneGet);
