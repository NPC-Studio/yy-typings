use crate::{ObjectOverrideProperty, ResourceVersion, Tags, ViewPath};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    /// Common data
    #[serde(flatten)]
    pub common_data: crate::CommonData<ConstGmRoom>,

    /// The relative subpath of the creation code for this room,
    /// if it exists.
    pub creation_code_file: String,

    pub inherit_code: bool,

    pub inherit_creation_order: bool,

    /// This is only meaningful if `parent_room` is `Some`.
    pub inherit_layers: bool,

    pub instance_creation_order: Vec<ViewPath>,

    /// Is this used in DragNDrop? Hopefully not! that would get messy.
    pub is_dnd: bool,

    /// The layers of data which are in the room.
    pub layers: Vec<RoomLayer>,

    pub parent: crate::ViewPath,

    /// The path of the parent room.
    pub parent_room: Option<ViewPath>,

    pub physics_settings: PhysicsSettings,

    pub room_settings: RoomSettings,

    pub sequence_id: Option<()>,

    pub tags: Tags,

    /// Eight (at least) views. Most users won't have anything
    /// meaningful here.
    pub views: Vec<RoomView>,

    pub view_settings: ViewSettings,

    /// A volume? I have no idea where this appears in the UI.
    /// Appears to be a number between 0.0 and 1.0.
    pub volume: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct RoomView {
    pub inherit: bool,
    pub visible: bool,
    pub xview: i32,
    pub yview: i32,
    pub wview: u32,
    pub hview: u32,
    pub xport: i32,
    pub yport: i32,
    pub wport: u32,
    pub hport: u32,
    pub hborder: u32,
    pub vborder: u32,
    pub hspeed: i32,
    pub vspeed: i32,

    #[serde(rename = "objectId")]
    pub object_id: Option<ViewPath>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RoomLayer {
    #[serde(flatten)]
    pub data: LayerData,

    pub visible: bool,
    pub depth: i32,
    #[serde(rename = "userdefinedDepth")]
    pub user_defined_depth: bool,
    pub grid_x: i32,
    pub grid_y: i32,
    pub layers: Vec<RoomLayer>,
    pub hierarchy_frozen: bool,

    /// The resource version of this yy file. At default 1.0.
    pub resource_version: ResourceVersion,

    /// The name of the object. This is the human readable name used in the IDE.
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(tag = "resourceType")]
pub enum LayerData {
    #[serde(rename = "GMRInstanceLayer")]
    Instance(Instances),
    #[serde(rename = "GMRTileLayer")]
    Tilemap(Tilemap),
    #[serde(rename = "GMRAssetLayer")]
    Asset(Assets),
    #[serde(rename = "GMRBackgroundLayer")]
    Background(BackgroundSprite),
    #[serde(rename = "GMRLayer")]
    Folder,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Assets {
    pub assets: Vec<Asset>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub sprite_id: Option<ViewPath>,
    pub head_position: f64,
    pub rotation: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub animation_speed: f64,
    #[serde(rename = "colour")]
    pub color: usize,
    pub inherited_item_id: Option<ViewPath>,
    pub frozen: bool,
    pub ignore: bool,
    pub inherit_item_settings: bool,
    pub x: f64,
    pub y: f64,
    pub resource_version: ResourceVersion,
    pub name: String,
    pub resource_type: ConstGMSpriteGraphic,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundSprite {
    pub sprite_id: Option<ViewPath>,
    #[serde(rename = "colour")]
    pub color: usize,
    pub x: i32,
    pub y: i32,
    pub htiled: bool,
    pub vtiled: bool,
    pub hspeed: f64,
    pub vspeed: f64,
    pub stretch: bool,
    #[serde(rename = "animationFPS")]
    pub animation_fps: f64,
    pub animation_speed_type: i32,
    #[serde(rename = "userdefinedAnimFPS")]
    pub user_defined_anim_fps: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Tilemap {
    pub tileset_id: Option<ViewPath>,
    pub x: i32,
    pub y: i32,
    pub tiles: TilemapTileData,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct TilemapTileData {
    #[serde(rename = "TileDataFormat")]
    pub tile_data_format: Option<usize>,
    #[serde(rename = "SerialiseWidth")]
    pub serialize_width: i32,
    #[serde(rename = "SerialiseHeight")]
    pub serialize_height: i32,
    #[serde(rename = "TileCompressedData")]
    pub tile_compressed_data: Option<Vec<i64>>,
    #[serde(rename = "TileSerialiseData")]
    pub tile_serialize_data: Option<Vec<usize>>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct Instances {
    pub instances: Vec<Instance>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    pub resource_type: ConstGmInstance,
    pub resource_version: ResourceVersion,
    pub name: String,
    pub properties: Vec<ObjectOverrideProperty>,
    pub is_dnd: bool,
    pub object_id: ViewPath,
    pub inherit_code: bool,
    pub has_creation_code: bool,
    #[serde(rename = "colour")]
    pub color: u32,
    pub rotation: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub image_index: i32,
    pub image_speed: f64,
    pub inherited_item_id: Option<ViewPath>,
    pub frozen: bool,
    pub ignore: bool,
    pub inherit_item_settings: bool,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmRoom {
    #[serde(rename = "GMRoom")]
    #[default]
    Const,
}

#[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmInstance {
    #[serde(rename = "GMRInstance")]
    #[default]
    Const,
}

#[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGMSpriteGraphic {
    #[serde(rename = "GMRSpriteGraphic")]
    #[default]
    Const,
}

#[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RoomSettings {
    pub inherit_room_settings: bool,
    #[serde(rename = "Width")]
    pub width: i32,
    #[serde(rename = "Height")]
    pub height: i32,
    pub persistent: bool,
}

#[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ViewSettings {
    pub inherit_view_settings: bool,
    pub enable_views: bool,
    pub clear_view_background: bool,
    pub clear_display_buffer: bool,
}

#[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PhysicsSettings {
    inherit_physics_settings: bool,
    #[serde(rename = "PhysicsWorld")]
    physics_world: bool,
    #[serde(rename = "PhysicsWorldGravityX")]
    physics_world_gravity_x: f64,
    #[serde(rename = "PhysicsWorldGravityY")]
    physics_world_gravity_y: f64,
    #[serde(rename = "PhysicsWorldPixToMetres")]
    physics_world_pix_to_meters: f64,
}
