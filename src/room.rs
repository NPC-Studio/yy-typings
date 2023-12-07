use crate::{CommonData, ObjectOverrideProperty, ResourceVersion, ViewPath};
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    /// Common data
    #[serde(flatten)]
    pub common_data: CommonData<consts::Room>,

    /// The relative subpath of the creation code for this room,
    /// if it exists.
    pub creation_code_file: String,

    pub inherit_code: bool,

    pub inherit_creation_order: bool,

    /// This is only meaningful if `parent_room` is `Some`.
    pub inherit_layers: bool,

    /// The order in which instances are created. Users can edit this.
    pub instance_creation_order: Vec<ViewPath>,

    /// Is this used in DragNDrop? Hopefully not! that would get messy.
    pub is_dnd: bool,

    /// The layers of data which are in the room.
    pub layers: Vec<RoomLayer>,

    /// The parent folder.
    pub parent: crate::ViewPath,

    /// The path of the parent room.
    pub parent_room: Option<ViewPath>,

    /// All of the irksome physics settings.
    pub physics_settings: PhysicsSettings,

    /// Specific room setting details
    pub room_settings: RoomSettings,

    /// This appears to always be null, so that's cool.
    pub sequence_id: Option<()>,

    /// The tags associated with this object.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub tags: Vec<String>,

    /// Eight (at least) views. Most users won't have anything
    /// meaningful here.
    pub views: Vec<RoomView>,

    /// Ah the great view settings!
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

#[derive(Debug, PartialEq, Clone)]
pub struct RoomLayer {
    pub data: LayerData,

    pub visible: bool,
    pub depth: i32,
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

#[derive(Debug, PartialEq, Clone)]
pub enum LayerData {
    Instance(Vec<Instance>),
    Tilemap(Tilemap),
    Asset(Vec<Asset>),
    Background(BackgroundSprite),
    Folder,
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
    pub resource_type: consts::SpriteGraphic,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct BackgroundSprite {
    pub sprite_id: Option<ViewPath>,
    pub color: usize,
    pub x: i32,
    pub y: i32,
    pub htiled: bool,
    pub vtiled: bool,
    pub hspeed: f64,
    pub vspeed: f64,
    pub stretch: bool,
    pub animation_fps: f64,
    pub animation_speed_type: i32,
    pub user_defined_anim_fps: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Tilemap {
    pub tiles: TilemapTileData,
    pub tileset_id: Option<ViewPath>,
    pub x: i32,
    pub y: i32,
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
#[serde(rename_all = "camelCase")]
pub struct Instance {
    pub resource_type: consts::Instance,
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
#[serde(rename_all = "PascalCase")]
pub struct PhysicsSettings {
    #[serde(rename = "inheritPhysicsSettings")]
    inherit_physics_settings: bool,
    physics_world: bool,
    physics_world_gravity_x: f64,
    physics_world_gravity_y: f64,
    physics_world_pix_to_meters: f64,
}

gm_const!(
    Room -> "GMRoom",
    Instance -> "GMRInstance",
    SpriteGraphic -> "GMRSpriteGraphic",
);

impl Serialize for RoomLayer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // TODO figure out the number for all possible states...
        let mut state = serializer.serialize_struct("RoomLayer", 1000)?;
        let resource_type = match &self.data {
            LayerData::Instance(_) => "GMRInstanceLayer",
            LayerData::Tilemap(_) => "GMRTileLayer",
            LayerData::Asset(_) => "GMRAssetLayer",
            LayerData::Background(_) => "GMRBackgroundLayer",
            LayerData::Folder => "GMRLayer",
        };

        state.serialize_field("resourceType", resource_type)?;
        state.serialize_field("resourceVersion", &self.resource_version)?;
        state.serialize_field("name", &self.name)?;

        match &self.data {
            LayerData::Asset(asset_list) => {
                state.serialize_field("assets", asset_list)?;
            }
            LayerData::Background(background) => {
                // "animationFPS":15.0,"animationSpeedType":0,"colour":4281991226
                state.serialize_field("animationFPS", &background.animation_fps)?;
                state.serialize_field("animationSpeedType", &background.animation_speed_type)?;
                state.serialize_field("colour", &background.color)?;
            }
            _ => {}
        }

        state.serialize_field("depth", &self.depth)?;
        state.serialize_field("effectEnabled", &false)?;
        state.serialize_field("effectType", &Option::<()>::None)?;
        state.serialize_field("effectType", &Option::<()>::None)?;
        state.serialize_field("gridX", &self.grid_x)?;
        state.serialize_field("gridY", &self.grid_y)?;
        state.serialize_field("hierarchyFrozen", &self.hierarchy_frozen)?;
        state.serialize_field("inheritLayerDepth", &false)?;
        state.serialize_field("inheritLayerSettings", &false)?;
        state.serialize_field("inheritSubLayers", &true)?;
        state.serialize_field("inheritVisibility", &true)?;

        if let LayerData::Instance(instances) = &self.data {
            state.serialize_field("instances", instances)?;
        }

        // I think everyone has layers...
        state.serialize_field("layers", &self.layers)?;
        state.serialize_field::<Vec<()>>("properties", &vec![])?;

        if let LayerData::Tilemap(tile_data) = &self.data {
            state.serialize_field("tiles", &tile_data.tiles)?;
            state.serialize_field("tilesetId", &tile_data.tileset_id)?;
        }

        state.serialize_field("userdefinedDepth", &self.user_defined_depth)?;
        state.serialize_field("visible", &self.visible)?;

        if let LayerData::Tilemap(tile_data) = &self.data {
            state.serialize_field("x", &tile_data.x)?;
            state.serialize_field("y", &tile_data.y)?;
        }
        state.end()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn basic_cycle() {
        fn t(input: &str) {
            let input = crate::TrailingCommaUtility::clear_trailing_comma_once(input);
            let room_data: super::Room = serde_json::from_str(&input).unwrap();
            println!("{}", crate::serialize_file(&room_data));

            let our_value = serde_json::to_value(room_data).unwrap();

            let their_value: serde_json::Value = serde_json::from_str(&input).unwrap();

            assert_eq!(our_value, their_value);
        }

        const TESTS: &[&str] = &[
            include_str!("./../data/rooms/rm_farm.yy"),
            include_str!("./../data/rooms/rm_inn.yy"),
            include_str!("./../data/rooms/rm_mines_upper_elevator10.yy"),
            include_str!("./../data/rooms/rm_town.yy"),
        ];

        for test in TESTS {
            t(test);
        }
    }
}
