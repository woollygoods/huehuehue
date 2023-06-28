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
#[serde(rename_all = "snake_case")]
pub enum HueV2ResourceType {
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
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
}
hue_v2_res_get!(HueV2ResourceGet);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HueV2ProductArchetype {
    BridgeV2,
    UnknownArchetype,
    ClassicBulb,
    SultanBulb,
    FloodBulb,
    SpotBulb,
    CandleBulb,
    LusterBulb,
    PendantRound,
    PendantLong,
    CeilingRound,
    CeilingSquare,
    FloorShade,
    FloorLantern,
    TableShade,
    RecessedCeiling,
    RecessedFloot,
    SingleSpot,
    DoubleSpot,
    TableWash,
    WallLantern,
    WallShade,
    FlexibleLamp,
    GroundSpot,
    WallSpot,
    Plug,
    HueGo,
    HueLightstrip,
    HueIris,
    HueBloom,
    Bollard,
    WallWasher,
    HuePlay,
    VintageBulb,
    VintageCandleBulb,
    EllipseBulb,
    TriangleBulb,
    SmallGlobeBulb,
    LargeGlobeBulb,
    EdisonBulb,
    ChristmasTree,
    StringLight,
    HueCentris,
    HueLightstripTv,
    HueLightstripPc,
    HueTube,
    HueSigne,
    PendantSpot,
    CeilingHorizontal,
    CeilingTube,
}

#[derive(Debug, Deserialize)]
pub struct HueV2DeviceGetProductData {
    model_id: String,
    manufacturer_name: String,
    product_name: String,
    product_archetype: HueV2ProductArchetype,
    certified: bool,
    software_version: String,
    hardware_platform_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct HueV2DeviceGetMetaData {
    name: String,
    archetype: HueV2ProductArchetype,
}

#[derive(Debug, Deserialize)]
pub struct HueV2ResourceIdentifierGet {
    rid: String,
    rtype: HueV2ResourceType,
}

#[derive(Debug, Deserialize)]
pub struct HueV2DeviceGet {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    product_data: HueV2DeviceGetProductData,
    metadata: HueV2DeviceGetMetaData,
    services: Vec<HueV2ResourceIdentifierGet>,
}
hue_v2_res_get!(HueV2DeviceGet);

#[derive(Debug, Deserialize)]
pub struct HueV2BridgeHomeGet {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    children: Vec<HueV2ResourceIdentifierGet>,
    services: Vec<HueV2ResourceIdentifierGet>,
}
hue_v2_res_get!(HueV2BridgeHomeGet);

#[derive(Debug, Deserialize)]
pub enum HueV2RoomArchetype {
    LivingRoom,
    Kitchen,
    Dining,
    Bedroom,
    KidsBedroom,
    Bathroom,
    Nursery,
    Recreation,
    Office,
    Gym,
    Hallway,
    Toilet,
    FrontDoor,
    Garage,
    Terrace,
    Garden,
    Driveway,
    Carport,
    Home,
    Downstairs,
    Upstairs,
    TopFloor,
    Attic,
    GuestRoom,
    Staircase,
    Lounge,
    ManCave,
    Computer,
    Studio,
    Music,
    Tv,
    Reading,
    Closet,
    Storage,
    LaundryRoom,
    Balcony,
    Porch,
    Barbecue,
    Pool,
    Other,
}

#[derive(Debug, Deserialize)]
pub struct HueV2RoomGetMetadata {
    name: String,
    archetype: HueV2RoomArchetype,
}

#[derive(Debug, Deserialize)]
pub struct HueV2RoomGet {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    children: Vec<HueV2ResourceIdentifierGet>,
    services: Vec<HueV2ResourceIdentifierGet>,
    metadata: HueV2RoomGetMetadata,
}
hue_v2_res_get!(HueV2RoomGet);

#[derive(Debug, Deserialize)]
pub struct HueV2ZoneGet {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    children: Vec<HueV2ResourceIdentifierGet>,
    services: Vec<HueV2ResourceIdentifierGet>,
    metadata: HueV2RoomGetMetadata,
}
hue_v2_res_get!(HueV2ZoneGet);

#[derive(Debug, Deserialize)]
pub enum HueV2LightArchetype {
    UnknownArchetype,
    ClassicBulb,
    SultanBulb,
    FloodBulb,
    SpotBulb,
    CandleBulb,
    LusterBulb,
    PendantRound,
    PendantLong,
    CeilingRound,
    CeilingSquare,
    FloorShade,
    FloorLantern,
    TableShade,
    RecessedCeiling,
    RecessedFloot,
    SingleSpot,
    DoubleSpot,
    TableWash,
    WallLantern,
    WallShade,
    FlexibleLamp,
    GroundSpot,
    WallSpot,
    Plug,
    HueGo,
    HueLightstrip,
    HueIris,
    HueBloom,
    Bollard,
    WallWasher,
    HuePlay,
    VintageBulb,
    VintageCandleBulb,
    EllipseBulb,
    TriangleBulb,
    SmallGlobeBulb,
    LargeGlobeBulb,
    EdisonBulb,
    ChristmasTree,
    StringLight,
    HueCentris,
    HueLightstripTv,
    HueLightstripPc,
    HueTube,
    HueSigne,
    PendantSpot,
    CeilingHorizontal,
    CeilingTube,
}
#[derive(Debug, Deserialize)]
pub struct HueV2LightGetMetadata {
    name: String,
    archetype: HueV2LightArchetype,
    fixed_mired: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetOn {
    on: bool,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetDimming {
    brightness: f64,
    min_dim_level: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetColorTemperatureMirekSchema {
    mirek_minimum: u32,
    mirek_maximum: u32,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetColorTemperature {
    mirek: u32,
    mirek_valid: bool,
    mirek_schema: HueV2LightGetColorTemperatureMirekSchema,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetColorXY {
    x: f64,
    y: f64,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetColorGamutChannel {
    x: f64,
    y: f64,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetColorGamut {
    red: HueV2LightGetColorGamutChannel,
    green: HueV2LightGetColorGamutChannel,
    blue: HueV2LightGetColorGamutChannel,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightGetColorGamutType {
    A,
    B,
    C,
    Other,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetColor {
    xy: HueV2LightGetColorXY,
    gamut: Option<HueV2LightGetColorGamut>,
    gamut_type: HueV2LightGetColorGamutType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightDynamicsStatus {
    DynamicPalette,
    None,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetDynamics {
    status: HueV2LightDynamicsStatus,
    status_values: Vec<String>,
    speed: f64,
    speed_valid: bool,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetAlert {
    action_values: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightSignal {
    NoSignal,
    OnOff,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetSignalingStatus {
    signal: HueV2LightSignal,
    estimated_end: String,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetSignaling {
    status: Option<HueV2LightGetSignalingStatus>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightMode {
    Normal,
    Streaming,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetGradientColor {
    xy: HueV2LightGetColorXY,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetGradientPoint {
    color: HueV2LightGetGradientColor,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightGradientMode {
    InterpolatedPalette,
    InterpolatedPaletteMirrored,
    RandomPixelated,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetGradient {
    points: Vec<HueV2LightGetGradientPoint>,
    mode: HueV2LightGradientMode,
    points_capable: u32,
    mode_values: Vec<String>,
    pixel_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightEffect {
    Sparkle,
    Fire,
    Candle,
    NoEffect,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetEffects {
    effect: HueV2LightEffect,
    status_values: Vec<String>,
    status: HueV2LightEffect,
    effect_values: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightTimedEffect {
    Sunrise,
    NoEffect,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetTimedEffects {
    effect: HueV2LightTimedEffect,
    duration: Option<u32>,
    status_values: Vec<String>,
    status: HueV2LightTimedEffect,
    effect_values: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightPowerupPreset {
    Safety,
    Powerfail,
    LastOnState,
    Custom,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightPowerupOnMode {
    On,
    Toggle,
    Previous,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetPowerupOn {
    mode: HueV2LightPowerupOnMode,
    on: Option<HueV2LightGetOn>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightPowerupDimmingMode {
    Dimming,
    Previous,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetPowerupDimmingBrightness {
    brightness: f64,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetPowerupDimming {
    mode: HueV2LightPowerupDimmingMode,
    dimming: Option<HueV2LightGetPowerupDimmingBrightness>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightColorTemperatureMode {
    ColorTemperature,
    Color,
    Previous,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetPowerupColorTemperature {
    mirek: u32,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetPowerupColorColor {
    xy: HueV2LightGetColorXY,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetPowerupColor {
    mode: HueV2LightColorTemperatureMode,
    color_temperature: Option<HueV2LightGetPowerupColorTemperature>,
    color: Option<HueV2LightGetPowerupColorColor>,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGetPowerup {
    preset: HueV2LightPowerupPreset,
    configured: bool,
    on: HueV2LightGetPowerupOn,
    dimming: Option<HueV2LightGetPowerupDimming>,
    color: Option<HueV2LightGetPowerupColor>,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightGet {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    owner: HueV2ResourceIdentifierGet,
    metadata: HueV2LightGetMetadata,
    on: HueV2LightGetOn,
    dimming: Option<HueV2LightGetDimming>,
    color_temperature: Option<HueV2LightGetColorTemperature>,
    color: Option<HueV2LightGetColor>,
    dynamics: Option<HueV2LightGetDynamics>,
    alert: Option<HueV2LightGetAlert>,
    signaling: Option<HueV2LightGetSignaling>,
    mode: HueV2LightMode,
    gradient: Option<HueV2LightGetGradient>,
    effects: Option<HueV2LightGetEffects>,
    timed_effects: Option<HueV2LightGetTimedEffects>,
    powerup: Option<HueV2LightGetPowerup>,
}
hue_v2_res_get!(HueV2LightGet);

#[derive(Debug, Deserialize)]
pub struct HueV2ButtonGetMetadata {
    control_id: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HueV2ButtonEvent {
    InitialPress,
    Repeat,
    ShortRelease,
    LongRelease,
    DoubleShortRelease,
    LongPress,
}

#[derive(Debug, Deserialize)]
pub struct HueV2ButtonGetButton {
    last_event: HueV2ButtonEvent,
}

#[derive(Debug, Deserialize)]
pub struct HueV2ButtonGet {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    owner: HueV2ResourceIdentifierGet,
    metadata: HueV2ButtonGetMetadata,
    button: HueV2ButtonGetButton,
}
hue_v2_res_get!(HueV2ButtonGet);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HueV2RelativeRotaryAction {
    Start,
    Repeat,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HueV2RelativeRotaryDirection {
    ClockWise,
    CounterClockWise,
}

#[derive(Debug, Deserialize)]
pub struct HueV2RelativeRotaryGetRelativeRotaryRotation {
    direction: HueV2RelativeRotaryDirection,
    steps: u32,
    duration: u32,
}

#[derive(Debug, Deserialize)]
pub struct HueV2RelativeRotaryGetRelativeRotaryEvent {
    action: HueV2RelativeRotaryAction,
    rotation: HueV2RelativeRotaryGetRelativeRotaryRotation,
}

#[derive(Debug, Deserialize)]
pub struct HueV2RelativeRotaryGetRelativeRotary {
    last_event: Option<HueV2RelativeRotaryGetRelativeRotaryEvent>,
}

#[derive(Debug, Deserialize)]
pub struct HueV2RelativeRotaryGet {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    owner: HueV2ResourceIdentifierGet,
    relative_rotary: HueV2RelativeRotaryGetRelativeRotary,
}
hue_v2_res_get!(HueV2RelativeRotaryGet);

#[derive(Debug, Deserialize)]
pub struct HueV2TemperatureGetTemperature {
    temperature: f64,
    temperature_valid: bool,
}

#[derive(Debug, Deserialize)]
pub struct HueV2TemperatureGet {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    owner: HueV2ResourceIdentifierGet,
    enabled: bool,
    temperature: HueV2TemperatureGetTemperature,
}
hue_v2_res_get!(HueV2TemperatureGet);

#[derive(Debug, Deserialize)]
pub struct HueV2LightLevelGetLevel {
    light_level: u32,
    light_level_valid: bool,
}

#[derive(Debug, Deserialize)]
pub struct HueV2LightLevelGet {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    owner: HueV2ResourceIdentifierGet,
    enabled: bool,
    light: HueV2LightLevelGetLevel,
}
hue_v2_res_get!(HueV2LightLevelGet);

#[derive(Debug, Deserialize)]
pub struct HueV2MotionGetMotion {
    motion: bool,
    motion_valid: bool,
}

#[derive(Debug, Deserialize)]
pub struct HueV2MotionGet {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    owner: HueV2ResourceIdentifierGet,
    enabled: bool,
    motion: HueV2MotionGetMotion,
}
hue_v2_res_get!(HueV2MotionGet);

#[derive(Debug, Deserialize)]
pub struct HueV2EntertainmentGetSegment {
    start: u32,
    length: u32,
}

#[derive(Debug, Deserialize)]
pub struct HueV2EntertainmentGetSegments {
    configurable: bool,
    max_segments: u32,
    segments: Vec<HueV2EntertainmentGetSegment>,
}

#[derive(Debug, Deserialize)]
pub struct HueV2EntertainmentGet {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    owner: HueV2ResourceIdentifierGet,
    renderer: bool,
    proxy: bool,
    equalizer: bool,
    max_streams: Option<u32>,
    segments: Option<HueV2EntertainmentGetSegments>,
}
hue_v2_res_get!(HueV2EntertainmentGet);

#[derive(Debug, Deserialize)]
pub struct HueV2GroupedLightGetOn {
    on: bool,
}

#[derive(Debug, Deserialize)]
pub struct HueV2GroupedLightGetDimming {
    dimming: f64,
}

#[derive(Debug, Deserialize)]
pub struct HueV2GroupedLightGetAlert {
    action_values: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct HueV2GroupedLightGet {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    owner: HueV2ResourceIdentifierGet,
    on: Option<HueV2GroupedLightGetOn>,
    dimming: Option<HueV2GroupedLightGetDimming>,
    alert: Option<HueV2GroupedLightGetAlert>,
}
hue_v2_res_get!(HueV2GroupedLightGet);

#[derive(Debug, Deserialize)]
pub enum HueV2DeviceBatteryState {
    Normal,
    Low,
    Critical,
}

#[derive(Debug, Deserialize)]
pub struct HueV2DevicePowerGetPowerState {
    battery_state: Option<HueV2DeviceBatteryState>,
    battery_level: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct HueV2DevicePowerGet {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    owner: HueV2ResourceIdentifierGet,
    power_state: HueV2DevicePowerGetPowerState,
}
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
