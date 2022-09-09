// use crate::{object_yy::ObjectOverrideProperty, ResourceVersion, Tags, ViewPath};
// use serde::{Deserialize, Serialize};
// use smart_default::SmartDefault;

// #[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Room {
//     /// Common data
//     #[serde(flatten)]
//     pub common_data: crate::CommonData<ConstGmRoom>,

//     /// Is this used in DragNDrop? Hopefully not! that would get messy.
//     pub is_dnd: bool,

//     /// A volume? I have no idea where this appears in the UI.
//     /// Appears to be a number between 0.0 and 1.0.
//     pub volume: f64,

//     /// The path of the parent room.
//     pub parent_room: Option<ViewPath>,

//     /// Eight (at least) views. Most users won't have anything
//     /// meaningful here.
//     pub views: Vec<RoomView>,

//     /// The layers of data which are in the room.
//     pub layers: Vec<Layer>,

//     /// This is only meaningful if `parent_room` is `Some`.
//     pub inherit_layers: bool,

//     /// The relative subpath of the creation code for this room,
//     /// if it exists.
//     pub creation_code_file: String,

//     pub inherit_code: bool,

//     pub instance_creation_order: Vec<ViewPath>,

//     pub inherit_creation_order: bool,

//     pub sequence_id: Option<()>,

//     pub room_settings: RoomSettings,

//     pub view_settings: ViewSettings,

//     pub physics_settings: PhysicsSettings,

//     pub parent: crate::ViewPath,
//     pub tags: Tags,
// }

// #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
// pub struct RoomView {
//     pub inherit: bool,
//     pub visible: bool,
//     pub xview: i32,
//     pub yview: i32,
//     pub wview: u32,
//     pub hview: u32,
//     pub xport: i32,
//     pub yport: i32,
//     pub wport: u32,
//     pub hport: u32,
//     pub hborder: u32,
//     pub vborder: u32,
//     pub hspeed: i32,
//     pub vspeed: i32,

//     #[serde(rename = "objectId")]
//     pub object_id: Option<ViewPath>,
// }

// #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Layer {
//     #[serde(flatten)]
//     pub data: LayerData,

//     pub visible: bool,
//     pub depth: i32,
//     #[serde(rename = "userdefinedDepth")]
//     pub user_defined_depth: bool,
//     pub inherit_layer_depth: bool,
//     pub inherit_layer_settings: bool,
//     pub grid_x: i32,
//     pub grid_y: i32,
//     pub layers: Vec<Layer>,
//     pub hierarchy_frozen: bool,

//     /// The resource version of this yy file. At default 1.0.
//     pub resource_version: ResourceVersion,

//     /// The name of the object. This is the human readable name used in the IDE.
//     pub name: String,

//     /// The tags given to the object.
//     pub tags: Tags,
// }

// #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
// #[serde(tag = "resourceType")]
// pub enum LayerData {
//     #[serde(rename = "GMRInstanceLayer")]
//     Instance(Instances),
//     #[serde(rename = "GMRTileLayer")]
//     Tilemap(Tilemap),
//     #[serde(rename = "GMRAssetLayer")]
//     Asset(Assets),
//     #[serde(rename = "GMRBackgroundLayer")]
//     Background(BackgroundSprite),
//     #[serde(rename = "GMRLayer")]
//     Folder,
// }

// #[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
// pub struct Assets {
//     pub assets: Vec<Asset>,
// }

// #[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
// #[serde(rename_all = "camelCase")]
// pub struct Asset {
//     pub sprite_id: Option<ViewPath>,
//     pub head_position: f64,
//     pub rotation: f64,
//     pub scale_x: f64,
//     pub scale_y: f64,
//     pub animation_speed: f64,
//     #[serde(rename = "colour")]
//     pub color: usize,
//     pub inherited_item_id: Option<ViewPath>,
//     pub frozen: bool,
//     pub ignore: bool,
//     pub inherit_item_settings: bool,
//     pub x: f64,
//     pub y: f64,
//     pub resource_version: ResourceVersion,
//     pub name: String,
//     pub tags: Tags,
//     pub resource_type: ConstGMSpriteGraphic,
// }

// #[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
// #[serde(rename_all = "camelCase")]
// pub struct BackgroundSprite {
//     pub sprite_id: Option<ViewPath>,
//     #[serde(rename = "colour")]
//     pub color: usize,
//     pub x: i32,
//     pub y: i32,
//     pub htiled: bool,
//     pub vtiled: bool,
//     pub hspeed: f64,
//     pub vspeed: f64,
//     pub stretch: bool,
//     #[serde(rename = "animationFPS")]
//     pub animation_fps: f64,
//     pub animation_speed_type: i32,
//     #[serde(rename = "userdefinedAnimFPS")]
//     pub user_defined_anim_fps: bool,
// }

// #[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
// #[serde(rename_all = "camelCase")]
// pub struct Tilemap {
//     pub tileset_id: Option<ViewPath>,
//     pub x: i32,
//     pub y: i32,
//     pub tiles: TilemapTileData,
// }

// #[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
// pub struct TilemapTileData {
//     #[serde(rename = "SerialiseWidth")]
//     pub serialize_width: i32,
//     #[serde(rename = "SerialiseHeight")]
//     pub serialize_height: i32,
//     #[serde(rename = "TileSerialiseData")]
//     pub tile_serialize_data: Vec<usize>,
// }

// #[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
// pub struct Instances {
//     instances: Vec<Instance>,
// }

// #[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Instance {
//     pub properties: Vec<ObjectOverrideProperty>,
//     pub is_dnd: bool,
//     pub object_id: ViewPath,
//     pub inherit_code: bool,
//     pub has_creation_code: bool,
//     #[serde(rename = "colour")]
//     pub color: u32,
//     pub rotation: f64,
//     pub scale_x: f64,
//     pub scale_y: f64,
//     pub image_index: i32,
//     pub image_speed: f64,
//     pub inherited_item_id: Option<ViewPath>,
//     pub frozen: bool,
//     pub ignore: bool,
//     pub inherit_item_settings: bool,
//     pub x: f64,
//     pub y: f64,
//     pub resource_version: ResourceVersion,
//     pub name: String,
//     pub tags: Tags,
//     pub resource_type: ConstGmInstance,
// }

// #[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
// pub enum ConstGmRoom {
//     #[serde(rename = "GMRoom")]
//     #[default]
//     Const,
// }

// #[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
// pub enum ConstGmInstance {
//     #[serde(rename = "GMRInstance")]
//     #[default]
//     Const,
// }

// #[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
// pub enum ConstGMSpriteGraphic {
//     #[serde(rename = "GMRSpriteGraphic")]
//     #[default]
//     Const,
// }

// #[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct RoomSettings {
//     pub inherit_room_settings: bool,
//     #[serde(rename = "Width")]
//     pub width: i32,
//     #[serde(rename = "Height")]
//     pub height: i32,
//     pub persistent: bool,
// }

// #[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct ViewSettings {
//     pub inherit_view_settings: bool,
//     pub enable_views: bool,
//     pub clear_view_background: bool,
//     pub clear_display_buffer: bool,
// }

// #[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct PhysicsSettings {
//     inherit_physics_settings: bool,
//     #[serde(rename = "PhysicsWorld")]
//     physics_world: bool,
//     #[serde(rename = "PhysicsWorldGravityX")]
//     physics_world_gravity_x: f64,
//     #[serde(rename = "PhysicsWorldGravityY")]
//     physics_world_gravity_y: f64,
//     #[serde(rename = "PhysicsWorldPixToMetres")]
//     physics_world_pix_to_meters: f64,
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::utils::TrailingCommaUtility;
//     use include_dir::{include_dir, Dir, DirEntry};
//     static ROOM_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/data/rooms");

//     #[test]
//     fn trivial_room() {
//         let tcu = TrailingCommaUtility::new();

//         for file in ROOM_DIR.find("**/*.yy").unwrap() {
//             if let DirEntry::File(file) = file {
//                 println!("parsing {}", file.path().display());
//                 let our_str = std::str::from_utf8(file.contents()).unwrap();
//                 let our_str = tcu.clear_trailing_comma(our_str);
//                 serde_json::from_str::<Room>(&our_str).unwrap();
//             }
//         }
//     }

//     /// This checks if we've sheered off any data in a serialization/deserialization cycle.
//     #[test]
//     fn room_sheer() {
//         let tcu = TrailingCommaUtility::new();

//         for file in ROOM_DIR.find("**/*.yy").unwrap() {
//             if let DirEntry::File(file) = file {
//                 println!("parsing {}", file.path().display());
//                 let our_str = std::str::from_utf8(file.contents()).unwrap();
//                 let our_str = tcu.clear_trailing_comma(our_str);
//                 let json_room: serde_json::Value = serde_json::from_str(&our_str).unwrap();
//                 let our_room = serde_json::from_value::<Room>(json_room.clone()).unwrap();
//                 let our_room_as_json = serde_json::to_value(our_room).unwrap();

//                 if our_room_as_json != json_room {
//                     println!("{}", serde_json::to_string(&our_room_as_json).unwrap());
//                     println!();
//                     println!("{}", serde_json::to_string(&json_room).unwrap());

//                     panic!("whoops");
//                 }
//                 // assert_eq!(our_room_as_json, json_room, "whoops not the same");
//             }
//         }
//     }

//     #[test]
//     fn room_string_sheer() {
//         let file_raw = include_str!("../../data/rooms/rm_western_ruins/rm_western_ruins.yy");

//         let file_parsed: Room =
//             serde_json::from_str(&TrailingCommaUtility::clear_trailing_comma_once(file_raw))
//                 .unwrap();

//         let json = serde_json::to_string(&file_parsed).unwrap();

//         if json != file_raw {
//             println!("{}", json);
//             println!();
//             println!("{}", file_raw);

//             panic!("whoops");
//         }
//     }

//     #[test]
//     fn room_instance_sheer() {
//         let txt = r#"{"instances":[{"properties":[{"propertyId":{"name":"","path":""},"objectId":{"name":"obj_light_tester","path":"objects/obj_light_tester/obj_light_tester.yy"},"value":"0","resourceVersion":"1.0","name":null,"tags":[],"resourceType":"GMOverriddenProperty"},{"propertyId":{"name":"","path":""},"objectId":{"name":"obj_light_tester","path":"objects/obj_light_tester/obj_light_tester.yy"},"value":"1","resourceVersion":"1.0","name":"","tags":[],"resourceType":"GMOverriddenProperty"}],"isDnd":false,"objectId":{"name":"obj_light_tester","path":"objects/obj_light_tester/obj_light_tester.yy"},"inheritCode":false,"hasCreationCode":false,"colour":4294967295,"rotation":0.0,"scaleX":1.0,"scaleY":1.0,"imageIndex":0,"imageSpeed":0.0,"inheritedItemId":null,"frozen":false,"ignore":false,"inheritItemSettings":false,"x":160.0,"y":32.0,"resourceVersion":"1.0","name":"inst_56DEEFC2","tags":[],"resourceType":"GMRInstance"}],"visible":true,"depth":0,"userdefinedDepth":false,"inheritLayerDepth":false,"inheritLayerSettings":false,"gridX":32,"gridY":32,"layers":[],"hierarchyFrozen":false,"resourceVersion":"1.0","name":"Instances","tags":[],"resourceType":"GMRInstanceLayer"}"#;

//         let parse: Layer = serde_json::from_str(txt).unwrap();
//         let output = serde_json::to_string(&parse).unwrap();

//         assert_eq!(txt, output);
//     }
// }
