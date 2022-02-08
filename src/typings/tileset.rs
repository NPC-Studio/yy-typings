use crate::{ResourceData, TexturePath, ViewPath};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

use super::resource_data::ResourceSubData;

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tileset {
    pub sprite_id: Option<ViewPath>,

    pub tile_width: u64,
    pub tile_height: u64,
    pub tilexoff: u64,
    pub tileyoff: u64,
    pub tilehsep: u64,
    pub tilevsep: u64,

    pub sprite_no_export: bool,
    pub texture_group_id: TexturePath,

    #[serde(rename = "out_tilehborder")]
    pub out_tile_hborder: u64,
    #[serde(rename = "out_tilevborder")]
    pub out_tile_vborder: u64,

    #[serde(rename = "out_columns")]
    pub out_columns: u64,
    #[serde(rename = "tile_count")]
    pub tile_count: u64,
    pub auto_tile_sets: Vec<AutoTileSet>,
    pub tile_animation_frames: Vec<TileAnimationFrame>,
    pub tile_animation_speed: f64,
    pub tile_animation: TileAnimation,
    pub macro_page_tiles: MacroPageTiles,

    /// Common resource data.
    #[serde(flatten)]
    pub resource_data: ResourceData,

    /// Const id tag of the shader, given by Gms2.
    pub resource_type: ConstGmTileSet,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
pub struct TileAnimation {
    #[serde(rename = "FrameData")]
    frame_data: Vec<usize>,
    #[serde(rename = "SerialiseFrameCount")]
    serialize_frame_count: usize,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
pub struct MacroPageTiles {
    #[serde(rename = "SerialiseWidth")]
    pub serialize_width: usize,
    #[serde(rename = "SerialiseHeight")]
    pub serialize_height: usize,
    #[serde(rename = "TileSerialiseData")]
    pub tile_serialize_data: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
pub struct AutoTileSet {
    tiles: Vec<usize>,
    closed_edge: bool,
    #[serde(flatten)]
    sub_data: ResourceSubData,
    #[serde(rename = "resourceType")]
    resource_type: ConstGmAutoTileSet,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TileAnimationFrame {
    pub frames: Vec<usize>,
    #[serde(flatten)]
    pub sub_data: ResourceSubData,
    pub resource_type: ConstGmTileAnimation,
}

#[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmTileSet {
    #[serde(rename = "GMTileSet")]
    #[default]
    Const,
}

#[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmTileAnimation {
    #[serde(rename = "GMTileAnimation")]
    #[default]
    Const,
}

#[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmAutoTileSet {
    #[serde(rename = "GMAutoTileSet")]
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
    fn trivial_tileset_parsing() {
        static DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/data/tilesets");
        let tcu = TrailingCommaUtility::new();

        for file in DIR.find("**/*.yy").unwrap() {
            if let DirEntry::File(file) = file {
                println!("parsing {}", file.path().display());
                let our_str = std::str::from_utf8(file.contents()).unwrap();
                let our_str = tcu.clear_trailing_comma(our_str);
                serde_json::from_str::<Tileset>(&our_str).unwrap();
            }
        }
    }

    // #[test]
    // fn deep_equality() {
    //     let file_raw = include_str!("../../data/shaders/sh_draw_light_to_screen.yy");

    //     let file_parsed: Shader =
    //         serde_json::from_str(&TrailingCommaUtility::clear_trailing_comma_once(file_raw))
    //             .unwrap();

    //     let script = Shader {
    //         shader_type: ShaderType::GlslEs,
    //         resource_data: ResourceData {
    //             parent: ViewPath {
    //                 name: "shaders".to_string(),
    //                 path: ViewPathLocation(
    //                     "folders/Objects/system/lighting/shaders.yy".to_string(),
    //                 ),
    //             },
    //             resource_version: ResourceVersion::default(),
    //             name: "sh_draw_light_to_screen".to_string(),
    //             tags: vec![],
    //         },
    //         resource_type: ConstGmTileSet::Const,
    //     };

    //     assert_eq!(file_parsed, script);
    // }
}
