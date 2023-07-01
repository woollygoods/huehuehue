#![allow(unused)]

use core::fmt::Debug;
use paste::paste;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Error {
    description: String,
}

pub trait HueV2Res: Debug + Type {}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Response<F: HueV2Res> {
    errors: Vec<HueV2Error>,
    data: Vec<F>,
}

macro_rules! hue_v2_res {
    ($v:ident) => {
        paste::paste! {
            impl HueV2Res for $v {}

            pub type [<$v Response>] = HueV2Response<$v>;
        }
    };
}

#[derive(Debug, Deserialize, Serialize, Type)]
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

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Resource {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
}
hue_v2_res!(HueV2Resource);

#[derive(Debug, Deserialize, Serialize, Type)]
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

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2DeviceProductData {
    model_id: String,
    manufacturer_name: String,
    product_name: String,
    product_archetype: HueV2ProductArchetype,
    certified: bool,
    software_version: String,
    hardware_platform_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2DeviceMetadata {
    name: String,
    archetype: HueV2ProductArchetype,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2ResourceIdentifier {
    rid: String,
    rtype: HueV2ResourceType,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Device {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    product_data: HueV2DeviceProductData,
    metadata: HueV2DeviceMetadata,
    services: Vec<HueV2ResourceIdentifier>,
}
hue_v2_res!(HueV2Device);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2BridgeHome {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    children: Vec<HueV2ResourceIdentifier>,
    services: Vec<HueV2ResourceIdentifier>,
}
hue_v2_res!(HueV2BridgeHome);

#[derive(Debug, Deserialize, Serialize, Type)]
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

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2RoomMetadata {
    name: String,
    archetype: HueV2RoomArchetype,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Room {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    children: Vec<HueV2ResourceIdentifier>,
    services: Vec<HueV2ResourceIdentifier>,
    metadata: HueV2RoomMetadata,
}
hue_v2_res!(HueV2Room);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Zone {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    children: Vec<HueV2ResourceIdentifier>,
    services: Vec<HueV2ResourceIdentifier>,
    metadata: HueV2RoomMetadata,
}
hue_v2_res!(HueV2Zone);

#[derive(Debug, Deserialize, Serialize, Type)]
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
#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightMetadata {
    name: String,
    archetype: HueV2LightArchetype,
    fixed_mired: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightOn {
    on: bool,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightDimming {
    brightness: f64,
    min_dim_level: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightColorTemperatureMirekSchema {
    mirek_minimum: u32,
    mirek_maximum: u32,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightColorTemperature {
    mirek: u32,
    mirek_valid: bool,
    mirek_schema: HueV2LightColorTemperatureMirekSchema,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightColorXY {
    x: f64,
    y: f64,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightColorGamutChannel {
    x: f64,
    y: f64,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightColorGamut {
    red: HueV2LightColorGamutChannel,
    green: HueV2LightColorGamutChannel,
    blue: HueV2LightColorGamutChannel,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightColorGamutType {
    A,
    B,
    C,
    Other,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightColor {
    xy: HueV2LightColorXY,
    gamut: Option<HueV2LightColorGamut>,
    gamut_type: HueV2LightColorGamutType,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightDynamicsStatus {
    DynamicPalette,
    None,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightDynamics {
    status: HueV2LightDynamicsStatus,
    status_values: Vec<String>,
    speed: f64,
    speed_valid: bool,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightAlert {
    action_values: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightSignal {
    NoSignal,
    OnOff,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightSignalingStatus {
    signal: HueV2LightSignal,
    estimated_end: String,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightSignaling {
    status: Option<HueV2LightSignalingStatus>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightMode {
    Normal,
    Streaming,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightGradientColor {
    xy: HueV2LightColorXY,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightGradientPoint {
    color: HueV2LightGradientColor,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightGradientMode {
    InterpolatedPalette,
    InterpolatedPaletteMirrored,
    RandomPixelated,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightGradient {
    points: Vec<HueV2LightGradientPoint>,
    mode: HueV2LightGradientMode,
    points_capable: u32,
    mode_values: Vec<String>,
    pixel_count: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightEffect {
    Sparkle,
    Fire,
    Candle,
    NoEffect,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightEffects {
    effect: HueV2LightEffect,
    status_values: Vec<String>,
    status: HueV2LightEffect,
    effect_values: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightTimedEffect {
    Sunrise,
    NoEffect,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightTimedEffects {
    effect: HueV2LightTimedEffect,
    duration: Option<u32>,
    status_values: Vec<String>,
    status: HueV2LightTimedEffect,
    effect_values: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightPowerupPreset {
    Safety,
    Powerfail,
    LastOnState,
    Custom,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightPowerupOnMode {
    On,
    Toggle,
    Previous,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightPowerupOn {
    mode: HueV2LightPowerupOnMode,
    on: Option<HueV2LightOn>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightPowerupDimmingMode {
    Dimming,
    Previous,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightPowerupDimmingBrightness {
    brightness: f64,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightPowerupDimming {
    mode: HueV2LightPowerupDimmingMode,
    dimming: Option<HueV2LightPowerupDimmingBrightness>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightColorTemperatureMode {
    ColorTemperature,
    Color,
    Previous,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightPowerupColorTemperature {
    mirek: u32,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightPowerupColorColor {
    xy: HueV2LightColorXY,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightPowerupColor {
    mode: HueV2LightColorTemperatureMode,
    color_temperature: Option<HueV2LightPowerupColorTemperature>,
    color: Option<HueV2LightPowerupColorColor>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightPowerup {
    preset: HueV2LightPowerupPreset,
    configured: bool,
    on: HueV2LightPowerupOn,
    dimming: Option<HueV2LightPowerupDimming>,
    color: Option<HueV2LightPowerupColor>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Light {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    owner: HueV2ResourceIdentifier,
    metadata: HueV2LightMetadata,
    on: HueV2LightOn,
    dimming: Option<HueV2LightDimming>,
    color_temperature: Option<HueV2LightColorTemperature>,
    color: Option<HueV2LightColor>,
    dynamics: Option<HueV2LightDynamics>,
    alert: Option<HueV2LightAlert>,
    signaling: Option<HueV2LightSignaling>,
    mode: HueV2LightMode,
    gradient: Option<HueV2LightGradient>,
    effects: Option<HueV2LightEffects>,
    timed_effects: Option<HueV2LightTimedEffects>,
    powerup: Option<HueV2LightPowerup>,
}
hue_v2_res!(HueV2Light);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2ButtonMetadata {
    control_id: u32,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2ButtonEvent {
    InitialPress,
    Repeat,
    ShortRelease,
    LongRelease,
    DoubleShortRelease,
    LongPress,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2ButtonButton {
    last_event: HueV2ButtonEvent,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Button {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    owner: HueV2ResourceIdentifier,
    metadata: HueV2ButtonMetadata,
    button: HueV2ButtonButton,
}
hue_v2_res!(HueV2Button);

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2RelativeRotaryAction {
    Start,
    Repeat,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2RelativeRotaryDirection {
    ClockWise,
    CounterClockWise,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2RelativeRotaryRotation {
    direction: HueV2RelativeRotaryDirection,
    steps: u32,
    duration: u32,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2RelativeRotaryEvent {
    action: HueV2RelativeRotaryAction,
    rotation: HueV2RelativeRotaryRotation,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2RelativeRotaryInner {
    last_event: Option<HueV2RelativeRotaryEvent>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2RelativeRotary {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    owner: HueV2ResourceIdentifier,
    relative_rotary: HueV2RelativeRotaryInner,
}
hue_v2_res!(HueV2RelativeRotary);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2TemperatureInner {
    temperature: f64,
    temperature_valid: bool,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Temperature {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    owner: HueV2ResourceIdentifier,
    enabled: bool,
    temperature: HueV2TemperatureInner,
}
hue_v2_res!(HueV2Temperature);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightLevelInner {
    light_level: u32,
    light_level_valid: bool,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightLevel {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    owner: HueV2ResourceIdentifier,
    enabled: bool,
    light: HueV2LightLevelInner,
}
hue_v2_res!(HueV2LightLevel);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2MotionInner {
    motion: bool,
    motion_valid: bool,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Motion {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    owner: HueV2ResourceIdentifier,
    enabled: bool,
    motion: HueV2MotionInner,
}
hue_v2_res!(HueV2Motion);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2EntertainmentSegment {
    start: u32,
    length: u32,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2EntertainmentSegments {
    configurable: bool,
    max_segments: u32,
    segments: Vec<HueV2EntertainmentSegment>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Entertainment {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    owner: HueV2ResourceIdentifier,
    renderer: bool,
    proxy: bool,
    equalizer: bool,
    max_streams: Option<u32>,
    segments: Option<HueV2EntertainmentSegments>,
}
hue_v2_res!(HueV2Entertainment);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2GroupedLightOn {
    on: bool,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2GroupedLightDimming {
    dimming: f64,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2GroupedLightAlert {
    action_values: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2GroupedLight {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    owner: HueV2ResourceIdentifier,
    on: Option<HueV2GroupedLightOn>,
    dimming: Option<HueV2GroupedLightDimming>,
    alert: Option<HueV2GroupedLightAlert>,
}
hue_v2_res!(HueV2GroupedLight);

#[derive(Debug, Deserialize, Serialize, Type)]
pub enum HueV2DeviceBatteryState {
    Normal,
    Low,
    Critical,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2DevicePowerState {
    battery_state: Option<HueV2DeviceBatteryState>,
    battery_level: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2DevicePower {
    id: String,
    r#type: HueV2ResourceType,
    id_v1: Option<String>,
    owner: HueV2ResourceIdentifier,
    power_state: HueV2DevicePowerState,
}
hue_v2_res!(HueV2DevicePower);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2ZigbeeBridgeConnectivity {}
hue_v2_res!(HueV2ZigbeeBridgeConnectivity);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2ZigbeeConnectivity {}
hue_v2_res!(HueV2ZigbeeConnectivity);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2ZgpConnectivity {}
hue_v2_res!(HueV2ZgpConnectivity);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Bridge {}
hue_v2_res!(HueV2Bridge);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2ZigbeeDeviceDiscovery {}
hue_v2_res!(HueV2ZigbeeDeviceDiscovery);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Homekit {}
hue_v2_res!(HueV2Homekit);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Matter {}
hue_v2_res!(HueV2Matter);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2MatterFabric {}
hue_v2_res!(HueV2MatterFabric);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Scene {}
hue_v2_res!(HueV2Scene);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2EntertainmentConfiguration {}
hue_v2_res!(HueV2EntertainmentConfiguration);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2PublicImage {}
hue_v2_res!(HueV2PublicImage);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2AuthV1 {}
hue_v2_res!(HueV2AuthV1);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2BehaviorScript {}
hue_v2_res!(HueV2BehaviorScript);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2BehaviorInstance {}
hue_v2_res!(HueV2BehaviorInstance);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Geofence {}
hue_v2_res!(HueV2Geofence);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2GeofenceClient {}
hue_v2_res!(HueV2GeofenceClient);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Geolocation {}
hue_v2_res!(HueV2Geolocation);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2SmartScene {}
hue_v2_res!(HueV2SmartScene);
