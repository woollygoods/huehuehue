#![allow(unused)]

use core::fmt::Debug;
use paste::paste;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Error {
    pub description: String,
}

pub trait HueV2Res: Debug + Type {}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Response<F: HueV2Res> {
    pub errors: Vec<HueV2Error>,
    pub data: Vec<F>,
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
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
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
    pub model_id: String,
    pub manufacturer_name: String,
    pub product_name: String,
    pub product_archetype: HueV2ProductArchetype,
    pub certified: bool,
    pub software_version: String,
    pub hardware_platform_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2DeviceMetadata {
    pub name: String,
    pub archetype: HueV2ProductArchetype,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2ResourceIdentifier {
    pub rid: String,
    pub rtype: HueV2ResourceType,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Device {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub product_data: HueV2DeviceProductData,
    pub metadata: HueV2DeviceMetadata,
    pub services: Vec<HueV2ResourceIdentifier>,
}
hue_v2_res!(HueV2Device);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2BridgeHome {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub children: Vec<HueV2ResourceIdentifier>,
    pub services: Vec<HueV2ResourceIdentifier>,
}
hue_v2_res!(HueV2BridgeHome);

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
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
    pub name: String,
    pub archetype: HueV2RoomArchetype,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Room {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub children: Vec<HueV2ResourceIdentifier>,
    pub services: Vec<HueV2ResourceIdentifier>,
    pub metadata: HueV2RoomMetadata,
}
hue_v2_res!(HueV2Room);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Zone {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub children: Vec<HueV2ResourceIdentifier>,
    pub services: Vec<HueV2ResourceIdentifier>,
    pub metadata: HueV2RoomMetadata,
}
hue_v2_res!(HueV2Zone);

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
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
    pub name: String,
    pub archetype: HueV2LightArchetype,
    pub fixed_mired: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightOn {
    pub on: bool,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightDimming {
    pub brightness: f64,
    pub min_dim_level: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightColorTemperatureMirekSchema {
    pub mirek_minimum: u32,
    pub mirek_maximum: u32,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightColorTemperature {
    pub mirek: u32,
    pub mirek_valid: bool,
    pub mirek_schema: HueV2LightColorTemperatureMirekSchema,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightColorXY {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightColorGamutChannel {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightColorGamut {
    pub red: HueV2LightColorGamutChannel,
    pub green: HueV2LightColorGamutChannel,
    pub blue: HueV2LightColorGamutChannel,
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
    pub xy: HueV2LightColorXY,
    pub gamut: Option<HueV2LightColorGamut>,
    pub gamut_type: HueV2LightColorGamutType,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightDynamicsStatus {
    DynamicPalette,
    None,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightDynamics {
    pub status: HueV2LightDynamicsStatus,
    pub status_values: Vec<String>,
    pub speed: f64,
    pub speed_valid: bool,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightAlert {
    pub action_values: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightSignal {
    NoSignal,
    OnOff,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightSignalingStatus {
    pub signal: HueV2LightSignal,
    pub estimated_end: String,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightSignaling {
    pub status: Option<HueV2LightSignalingStatus>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightMode {
    Normal,
    Streaming,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightGradientColor {
    pub xy: HueV2LightColorXY,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightGradientPoint {
    pub color: HueV2LightGradientColor,
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
    pub points: Vec<HueV2LightGradientPoint>,
    pub mode: HueV2LightGradientMode,
    pub points_capable: u32,
    pub mode_values: Vec<String>,
    pub pixel_count: Option<u32>,
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
    pub effect: HueV2LightEffect,
    pub status_values: Vec<String>,
    pub status: HueV2LightEffect,
    pub effect_values: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightTimedEffect {
    Sunrise,
    NoEffect,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightTimedEffects {
    pub effect: HueV2LightTimedEffect,
    pub duration: Option<u32>,
    pub status_values: Vec<String>,
    pub status: HueV2LightTimedEffect,
    pub effect_values: Vec<String>,
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
    pub mode: HueV2LightPowerupOnMode,
    pub on: Option<HueV2LightOn>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2LightPowerupDimmingMode {
    Dimming,
    Previous,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightPowerupDimmingBrightness {
    pub brightness: f64,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightPowerupDimming {
    pub mode: HueV2LightPowerupDimmingMode,
    pub dimming: Option<HueV2LightPowerupDimmingBrightness>,
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
    pub mirek: u32,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightPowerupColorColor {
    pub xy: HueV2LightColorXY,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightPowerupColor {
    pub mode: HueV2LightColorTemperatureMode,
    pub color_temperature: Option<HueV2LightPowerupColorTemperature>,
    pub color: Option<HueV2LightPowerupColorColor>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightPowerup {
    pub preset: HueV2LightPowerupPreset,
    pub configured: bool,
    pub on: HueV2LightPowerupOn,
    pub dimming: Option<HueV2LightPowerupDimming>,
    pub color: Option<HueV2LightPowerupColor>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Light {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub owner: HueV2ResourceIdentifier,
    pub metadata: HueV2LightMetadata,
    pub on: HueV2LightOn,
    pub dimming: Option<HueV2LightDimming>,
    pub color_temperature: Option<HueV2LightColorTemperature>,
    pub color: Option<HueV2LightColor>,
    pub dynamics: Option<HueV2LightDynamics>,
    pub alert: Option<HueV2LightAlert>,
    pub signaling: Option<HueV2LightSignaling>,
    pub mode: HueV2LightMode,
    pub gradient: Option<HueV2LightGradient>,
    pub effects: Option<HueV2LightEffects>,
    pub timed_effects: Option<HueV2LightTimedEffects>,
    pub powerup: Option<HueV2LightPowerup>,
}
hue_v2_res!(HueV2Light);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2ButtonMetadata {
    pub control_id: u32,
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
    pub last_event: HueV2ButtonEvent,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Button {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub owner: HueV2ResourceIdentifier,
    pub metadata: HueV2ButtonMetadata,
    pub button: HueV2ButtonButton,
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
    pub direction: HueV2RelativeRotaryDirection,
    pub steps: u32,
    pub duration: u32,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2RelativeRotaryEvent {
    pub action: HueV2RelativeRotaryAction,
    pub rotation: HueV2RelativeRotaryRotation,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2RelativeRotaryInner {
    pub last_event: Option<HueV2RelativeRotaryEvent>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2RelativeRotary {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub owner: HueV2ResourceIdentifier,
    pub relative_rotary: HueV2RelativeRotaryInner,
}
hue_v2_res!(HueV2RelativeRotary);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2TemperatureInner {
    pub temperature: f64,
    pub temperature_valid: bool,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Temperature {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub owner: HueV2ResourceIdentifier,
    pub enabled: bool,
    pub temperature: HueV2TemperatureInner,
}
hue_v2_res!(HueV2Temperature);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightLevelInner {
    pub light_level: u32,
    pub light_level_valid: bool,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2LightLevel {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub owner: HueV2ResourceIdentifier,
    pub enabled: bool,
    pub light: HueV2LightLevelInner,
}
hue_v2_res!(HueV2LightLevel);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2MotionInner {
    pub motion: bool,
    pub motion_valid: bool,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Motion {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub owner: HueV2ResourceIdentifier,
    pub enabled: bool,
    pub motion: HueV2MotionInner,
}
hue_v2_res!(HueV2Motion);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2EntertainmentSegment {
    pub start: u32,
    pub length: u32,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2EntertainmentSegments {
    pub configurable: bool,
    pub max_segments: u32,
    pub segments: Vec<HueV2EntertainmentSegment>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Entertainment {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub owner: HueV2ResourceIdentifier,
    pub renderer: bool,
    pub proxy: bool,
    pub equalizer: bool,
    pub max_streams: Option<u32>,
    pub segments: Option<HueV2EntertainmentSegments>,
}
hue_v2_res!(HueV2Entertainment);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2GroupedLightOn {
    pub on: bool,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2GroupedLightDimming {
    pub dimming: f64,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2GroupedLightAlert {
    pub action_values: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2GroupedLight {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub owner: HueV2ResourceIdentifier,
    pub on: Option<HueV2GroupedLightOn>,
    pub dimming: Option<HueV2GroupedLightDimming>,
    pub alert: Option<HueV2GroupedLightAlert>,
}
hue_v2_res!(HueV2GroupedLight);

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2DeviceBatteryState {
    Normal,
    Low,
    Critical,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2DevicePowerState {
    pub battery_state: Option<HueV2DeviceBatteryState>,
    pub battery_level: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2DevicePower {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub owner: HueV2ResourceIdentifier,
    pub power_state: HueV2DevicePowerState,
}
hue_v2_res!(HueV2DevicePower);

#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2ZigbeeConnectivityStatus {
    Connected,
    Disconnected,
    ConnectivityIssue,
    UnidirectionalIncoming,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2ZigbeeConnectivity {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub owner: HueV2ResourceIdentifier,
    pub status: HueV2ZigbeeConnectivityStatus,
    pub mac_address: String,
}
hue_v2_res!(HueV2ZigbeeConnectivity);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2ZgpConnectivity {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub owner: HueV2ResourceIdentifier,
    pub status: HueV2ZigbeeConnectivityStatus,
    pub source_id: String,
}
hue_v2_res!(HueV2ZgpConnectivity);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2BridgeTimeZone {
    pub time_zone: String,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Bridge {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub owner: HueV2ResourceIdentifier,
    pub bridge_id: String,
    pub time_zone: HueV2BridgeTimeZone,
}
hue_v2_res!(HueV2Bridge);

#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2ZigbeeDeviceDiscoveryStatus {
    Active,
    Ready,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2ZigbeeDeviceDiscovery {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub owner: HueV2ResourceIdentifier,
    pub status: HueV2ZigbeeDeviceDiscoveryStatus,
}
hue_v2_res!(HueV2ZigbeeDeviceDiscovery);

#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2HomekitStatus {
    Paired,
    Pairing,
    Unpaired,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Homekit {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub status: HueV2HomekitStatus,
}
hue_v2_res!(HueV2Homekit);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Matter {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub max_fabrics: i32,
    pub has_qr_code: bool,
}
hue_v2_res!(HueV2Matter);

#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2MatterFabricStatus {
    Pending,
    Timedout,
    Paired,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2MatterFabricData {
    pub label: String,
    pub vendor_id: i32,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2MatterFabric {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub status: HueV2MatterFabricStatus,
    pub fabric_data: Option<HueV2MatterFabricData>,
    pub creation_time: String,
}
hue_v2_res!(HueV2MatterFabric);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2SceneActionTarget {
    pub rid: String,
    pub rtype: HueV2ResourceType,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2SceneActionOn {
    pub on: bool,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2SceneActionDimming {
    pub brightness: f64,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2SceneActionColorXY {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2SceneActionColor {
    pub xy: HueV2SceneActionColorXY,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2SceneActionColorTemperature {
    pub mirek: u32,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2SceneActionGradientMode {
    InterpolatedPalette,
    InterpolatedPaletteMirrored,
    RandomPixelated,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2SceneActionGradientPoint {
    pub color: HueV2SceneActionColor,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2SceneActionGradient {
    pub points: Vec<HueV2SceneActionGradientPoint>,
    pub mode: HueV2SceneActionGradientMode,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum HueV2SceneActionEffect {
    Sparkle,
    Fire,
    Candle,
    NoEffect,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2SceneActionEffects {
    pub effect: HueV2SceneActionEffect,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2SceneActionDynamics {
    pub duration: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2SceneActionInner {
    pub on: Option<HueV2SceneActionOn>,
    pub dimming: Option<HueV2SceneActionDimming>,
    pub color: Option<HueV2SceneActionColor>,
    pub color_temperature: Option<HueV2SceneActionColorTemperature>,
    pub gradient: Option<HueV2SceneActionGradient>,
    pub effects: Option<HueV2SceneActionEffects>,
    pub dynamics: Option<HueV2SceneActionDynamics>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2SceneAction {
    pub target: HueV2SceneActionTarget,
    pub action: HueV2SceneActionInner,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2SceneImage {
    pub rid: String,
    pub rtype: HueV2ResourceType,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2SceneMetadata {
    pub name: String,
    pub image: Option<HueV2SceneImage>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2SceneGroup {
    pub rid: String,
    pub rtype: HueV2ResourceType,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2ScenePaletteColorInnerXY {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2ScenePaletteColorInner {
    pub xy: HueV2ScenePaletteColorInnerXY,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2ScenePaletteColorDimming {
    pub brightness: f64,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2ScenePaletteColor {
    pub color: HueV2ScenePaletteColorInner,
    pub dimming: HueV2ScenePaletteColorDimming,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2ScenePaletteDimming {
    pub brightness: f64,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2ScenePaletteColorTemperatureInner {
    pub mirek: u32,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2ScenePaletteColorTemperatureDimming {
    pub brightness: f64,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2ScenePaletteColorTemperature {
    pub color_temperature: HueV2ScenePaletteColorTemperatureInner,
    pub dimming: HueV2ScenePaletteColorTemperatureDimming,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2ScenePalette {
    pub color: Vec<HueV2ScenePaletteColor>,
    pub dimming: Vec<HueV2ScenePaletteDimming>,
    pub color_temperature: Vec<HueV2ScenePaletteColorTemperature>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Scene {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub actions: Vec<HueV2SceneAction>,
    pub metadata: HueV2SceneMetadata,
    pub group: HueV2SceneGroup,
    pub palette: Option<HueV2ScenePalette>,
    pub speed: f64,
    pub auto_dynamic: bool,
}
hue_v2_res!(HueV2Scene);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2EntertainmentConfiguration {
    pub string: String,
}
hue_v2_res!(HueV2EntertainmentConfiguration);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2BehaviorScript {
    pub string: String,
}
hue_v2_res!(HueV2BehaviorScript);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2BehaviorInstance {
    pub string: String,
}
hue_v2_res!(HueV2BehaviorInstance);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2GeofenceClient {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub name: String,
}
hue_v2_res!(HueV2GeofenceClient);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2Geolocation {
    pub id: String,
    pub r#type: HueV2ResourceType,
    pub id_v1: Option<String>,
    pub is_configured: bool,
}
hue_v2_res!(HueV2Geolocation);

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct HueV2SmartScene {
    pub string: String,
}
hue_v2_res!(HueV2SmartScene);
