use crate::{CommonData, TexturePath, VersionStamp, ViewPath};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TileSet {
    /// The event version that GM uses -- it's currently "v1"
    #[serde(rename = "$GMTileSet")]
    pub gm_version_stamp: VersionStamp<1>,

    #[serde(flatten)]
    pub common_data: CommonData<consts::TileSet>,

    pub auto_tile_sets: Vec<AutoTileSet>,

    pub macro_page_tiles: MacroPageTiles,

    #[serde(rename = "out_columns")]
    pub out_columns: u64,

    #[serde(rename = "out_tilehborder")]
    pub out_tile_hborder: u64,
    #[serde(rename = "out_tilevborder")]
    pub out_tile_vborder: u64,

    pub parent: crate::ViewPath,

    pub sprite_id: Option<ViewPath>,
    pub sprite_no_export: bool,

    pub texture_group_id: TexturePath,

    pub tile_animation_frames: Vec<TileAnimationFrame>,
    pub tile_animation_speed: f64,

    pub tile_height: u64,
    pub tilehsep: u64,
    pub tilevsep: u64,
    pub tile_width: u64,
    pub tilexoff: u64,
    pub tileyoff: u64,

    #[serde(rename = "tile_count")]
    pub tile_count: u64,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub struct MacroPageTiles {
    #[serde(rename = "SerialiseHeight")]
    pub serialize_height: usize,
    #[serde(rename = "SerialiseWidth")]
    pub serialize_width: usize,
    #[serde(rename = "TileSerialiseData")]
    pub tile_serialize_data: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub struct AutoTileSet {
    #[serde(flatten)]
    common_data: CommonData<consts::AutoTileSet>,
    closed_edge: bool,
    tiles: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TileAnimationFrame {
    #[serde(flatten)]
    common_data: CommonData<consts::TileAnimation>,

    pub frames: Vec<usize>,
}

gm_const!(
    TileSet -> "GMTileSet",
    TileAnimation -> "GMTileAnimation",
    AutoTileSet -> "GMAutoTileSet",
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{utils::TrailingCommaUtility, ViewPath, ViewPathLocation};
    use pretty_assertions::assert_eq;

    #[test]
    fn deep_equality() {
        let file_raw = include_str!("../data/tileset/test.yy");

        let file_parsed: TileSet =
            serde_json::from_str(&TrailingCommaUtility::clear_trailing_comma_once(file_raw))
                .unwrap();

        let tset = TileSet {
            common_data: CommonData::new("tile_collision_info".to_string()),
            gm_version_stamp: VersionStamp,
            auto_tile_sets: vec![],
            macro_page_tiles: MacroPageTiles {
                serialize_height: 0,
                serialize_width: 0,
                tile_serialize_data: vec![],
            },
            out_columns: 3,
            out_tile_hborder: 2,
            out_tile_vborder: 2,
            parent: ViewPath {
                name: "__META".to_string(),
                path: ViewPathLocation::new("folders/Tile Sets/Tiles/__META.yy"),
            },
            sprite_id: Some(ViewPath {
                name: "spr_collision_tile_info".to_string(),
                path: ViewPathLocation::new(
                    "sprites/spr_collision_tile_info/spr_collision_tile_info.yy",
                ),
            }),
            sprite_no_export: true,
            texture_group_id: TexturePath {
                name: "Default".to_string(),
                path: crate::TexturePathLocation("texturegroups/Default".to_string()),
            },
            tile_animation_frames: vec![],
            tile_animation_speed: 15.0,
            tile_height: 8,
            tilehsep: 0,
            tilevsep: 0,
            tile_width: 8,
            tilexoff: 0,
            tileyoff: 0,
            tile_count: 7,
        };

        assert_eq!(file_parsed, tset);
    }
}
