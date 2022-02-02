use crate::{object_yy::ObjectOverrideProperty, ResourceData, ResourceVersion, Tags, ViewPath};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    // /// Is this used in DragNDrop? Hopefully not! that would get messy.
    // pub is_dn_d: bool,

    // /// A volume? I have no idea where this appears in the UI.
    // /// Appears to be a number between 0.0 and 1.0.
    // pub volume: f32,

    // /// The path of the parent room.
    // pub parent_room: Option<ViewPath>,

    // /// Eight (at least) views. Most users won't have anything
    // /// meaningful here.
    // pub views: Vec<()>,
    /// The layers of data which are in the room.
    pub layers: Vec<Layer>,

    // /// This is only meaningful if `parent_room` is `Some`.
    // pub inherit_layers: bool,

    // /// The relative subpath of the creation code for this room,
    // /// if it exists.
    // pub creation_code_file: String,
    /// Common resource data
    #[serde(flatten)]
    pub resource_data: ResourceData,

    /// Const id tag of the shader, given by Gms2.
    pub resource_type: ConstGmRoom,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    #[serde(flatten)]
    pub data: LayerData,

    pub visible: bool,
    pub depth: i32,
    pub userdefined_depth: bool,
    pub inherit_layer_depth: bool,
    pub inherit_layer_settings: bool,
    pub grid_x: i32,
    pub grid_y: i32,
    pub layers: Vec<Layer>,
    pub hierarchy_frozen: bool,

    /// The resource version of this yy file. At default 1.0.
    pub resource_version: ResourceVersion,

    /// The name of the object. This is the human readable name used in the IDE.
    pub name: String,

    /// The tags given to the object.
    pub tags: Tags,
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
    Background(Asset),
    #[serde(rename = "GMRLayer")]
    Folder,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Assets {
    assets: Vec<Asset>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    sprite_id: Option<ViewPath>,
    // head_position: f32,
    // rotation: f32,
    // scale_x: f32,
    // scale_y: f32,
    // animation_speed: f32,
    // color: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Tilemap {
    // pub tileset_id: ViewPath,
// pub x: i32,
// pub y: i32,
// pub tiles: TilemapTileData,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct TilemapTileData {
    #[serde(rename = "SerialiseWidth")]
    pub serialize_width: i32,
    #[serde(rename = "SerialiseHeight")]
    pub serialize_height: i32,
    #[serde(rename = "TileSerialiseData")]
    pub tile_serialize_data: Vec<usize>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct Instances {
    instances: Vec<Instance>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    pub properties: Vec<ObjectOverrideProperty>,
    pub is_dnd: bool,
    pub object_id: ViewPath,
    pub inherit_code: bool,
    pub has_creation_code: bool,
    #[serde(rename = "colour")]
    pub color: u32,
    pub rotation: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub image_index: i32,
    pub image_speed: f32,
    pub inherited_item_id: Option<()>,
    pub frozen: bool,
    pub ignore: bool,
    pub inherit_item_settings: bool,
    pub x: f32,
    pub y: f32,
    pub resource_version: ResourceVersion,
    pub name: String,
    pub tags: Tags,
    pub resource_type: ConstGmInstance,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::TrailingCommaUtility;
    use include_dir::{include_dir, Dir, DirEntry};
    // use pretty_assertions::assert_eq;

    #[test]
    fn trivial_room() {
        static DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/data/rooms");
        let tcu = TrailingCommaUtility::new();

        for file in DIR.find("**/*.yy").unwrap() {
            if let DirEntry::File(file) = file {
                println!("parsing {}", file.path().display());
                let our_str = std::str::from_utf8(file.contents()).unwrap();
                let our_str = tcu.clear_trailing_comma(our_str);
                serde_json::from_str::<Room>(&our_str).unwrap();
            }
        }
    }

    // #[test]
    // fn deep_equality() {
    //     let file_raw = include_str!("../../data/reo.yy");

    //     let file_parsed: Shader =
    //         serde_json::from_str(&TrailingCommaUtility::clear_trailing_comma_once(file_raw))
    //             .unwrap();

    //     let script = Shader {
    //         shader_type: ShaderType::GlslEs,
    //         parent: ViewPath {
    //             name: "shaders".to_string(),
    //             path: ViewPathLocation("folders/Objects/system/lighting/shaders.yy".to_string()),
    //         },
    //         resource_version: ResourceVersion::default(),
    //         name: "sh_draw_light_to_screen".to_string(),
    //         tags: vec![],
    //         resource_type: ConstGmShader::Const,
    //     };

    //     assert_eq!(file_parsed, script);
    // }
}
