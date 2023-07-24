use super::{sprite_constants::*, FrameId, Layer, ResourceVersion, SpriteSequence, TexturePath};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_default::SmartDefault;
use std::num::NonZeroUsize;

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sprite {
    #[serde(flatten)]
    pub common_data: crate::CommonData<ConstGmSprite>,

    #[serde(rename = "bbox_bottom")]
    pub bbox_bottom: i32,
    #[serde(rename = "bbox_left")]
    pub bbox_left: i32,
    #[serde(rename = "bbox_right")]
    pub bbox_right: i32,
    #[serde(rename = "bbox_top")]
    pub bbox_top: i32,
    pub bbox_mode: BBoxMode,

    pub collision_kind: CollisionKind,
    pub collision_tolerance: u8,

    #[serde(rename = "DynamicTexturePage")]
    pub dynamic_texture_page: bool,

    pub edge_filtering: bool,
    #[serde(rename = "For3D")]
    pub for3d: bool,

    /// Each frame within this Sprite File.
    pub frames: Vec<crate::CommonData<ConstGmSpriteFrame, FrameId, 1, 1>>,

    pub grid_x: usize,
    pub grid_y: usize,

    #[default(NonZeroUsize::new(1).unwrap())]
    pub height: NonZeroUsize,

    #[serde(rename = "HTile")]
    pub h_tile: bool,

    /// Each layer within each Frame for this Sprite File. Unless users use
    /// the built-in Sprite Editor, this will likely always have a .len() of
    /// 1, as there is always *one* layer. Layers are shared between frames.
    pub layers: Vec<Layer>,

    /// Optional nineslice data.
    pub nine_slice: Option<NineSlice>,

    pub origin: Origin,

    pub parent: crate::ViewPath,

    pub pre_multiply_alpha: bool,

    /// The sequence assigned to each sprite.
    pub sequence: SpriteSequence,

    /// This is probably always null, unless you make a swatch,
    /// but why are you doing that! Just don't do that. Easy.
    pub swatch_colours: serde_json::Value,

    /// The precision for Vector sprites. Its default is `2.525`, a number
    /// which is very odd in my opinion.
    #[default(2.525)]
    pub swf_precision: f64,

    /// This is the Path to the Texture Group Id.
    pub texture_group_id: TexturePath,

    /// The type of sprite, whether a bitmap or a vector sprite.
    /// Right now, we don't handle this intelligently.
    #[serde(rename = "type")]
    pub resource_sprite_type: usize,

    #[serde(rename = "VTile")]
    pub v_tile: bool,

    #[default(NonZeroUsize::new(1).unwrap())]
    pub width: NonZeroUsize,
}

#[derive(
    Serialize_repr,
    Deserialize_repr,
    PartialEq,
    Debug,
    SmartDefault,
    Copy,
    Clone,
    Eq,
    Ord,
    PartialOrd,
)]
#[repr(u8)]
pub enum CollisionKind {
    Precise,
    #[default]
    Rectangle,
    Ellipse,
    Diamond,
    PrecisePerFrame,
    RotatedRectangle,
}

#[derive(
    Serialize_repr,
    Deserialize_repr,
    PartialEq,
    Debug,
    SmartDefault,
    Copy,
    Clone,
    Eq,
    Ord,
    PartialOrd,
)]
#[repr(u8)]
pub enum BBoxMode {
    #[default]
    Automatic,
    FullImage,
    Manual,
}

#[derive(
    Serialize_repr,
    Deserialize_repr,
    PartialEq,
    Debug,
    SmartDefault,
    Copy,
    Clone,
    Eq,
    Ord,
    PartialOrd,
)]
#[repr(u8)]
pub enum Origin {
    #[default]
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Custom,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NineSlice {
    /// A constant, always "GMNineSliceData"
    pub resource_type: ConstGmNineSliceData,

    /// Version string. Right now, this is loosely typed and ignored,
    /// but will be used in the future to aid in parsing.
    pub resource_version: ResourceVersion,

    /// The bottom bound.
    pub bottom: u64,

    /// Whether nine-slicing is enabled. This data structure as a whole
    /// *may* be set to `null`, or be defined but disabled. This allows the user
    /// to toggle nine-slicing on and off.
    pub enabled: bool,

    /// The guide color for each bound, listed in this order:
    /// 0: Left
    /// 1: Top
    /// 2: Right
    /// 3: Bottom
    pub guide_colour: [GmEncodedColor; 4],

    /// The highlight color to use for the highlighted segment. Why in the world is this
    /// user configurable? But it is!
    pub highlight_colour: GmEncodedColor,

    /// The style of the highlight.
    pub highlight_style: HighlightStyle,

    /// The left bound.
    pub left: u64,

    /// The right bound.
    pub right: u64,

    /// This is an array of 5 tile mode sprites. I am leaving this typed as
    /// a raw array right now, rather than writing a custom serde impl, due to time
    /// constraints. In the future, I will make this into a nice struct.
    /// The following are each index:
    /// 0: Left
    /// 1: Top,
    /// 2: Right
    /// 3: Bottom
    /// 4: Center
    pub tile_mode: [TileMode; 5],

    /// The top bound.
    pub top: u64,
}

/// A color encoded in a u64 in BGR format. It does not contain Alpha.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone)]
#[repr(transparent)]
#[serde(transparent)]
pub struct GmEncodedColor(u64);

/// The highlight style for nineslice support.
#[derive(Debug, Serialize_repr, Deserialize_repr, SmartDefault, PartialEq, Eq, Clone)]
#[repr(u8)]
pub enum HighlightStyle {
    /// This inverts the colors below it.
    #[default]
    Inverted,
    /// This uses additive blending.
    Overlay,
}

/// The tilemodes for each nine-slice. Refer to GM's documentation on what each does.
#[derive(Debug, Serialize_repr, Deserialize_repr, SmartDefault, PartialEq, Eq, Clone)]
#[repr(u8)]
pub enum TileMode {
    #[default]
    Stretch,
    Repeat,
    Mirror,
    BlankRepeat,
    Hide,
}

// #[cfg(test)]
// mod tests {
//     use crate::{sprite_yy::*, utils::TrailingCommaUtility};
//     use include_dir::{include_dir, Dir, DirEntry};
//     use pretty_assertions::assert_eq;
//     use std::{num::NonZeroUsize, path::Path};

//     #[test]
//     fn trivial_sprite_parsing() {
//         let all_sprites: Dir = include_dir!("data/sprites");
//         let tcu = TrailingCommaUtility::new();

//         for sprite_file in all_sprites.find("**/*.yy").unwrap() {
//             if let DirEntry::File(file) = sprite_file {
//                 let our_str = std::str::from_utf8(file.contents()).unwrap();
//                 let our_str = tcu.clear_trailing_comma(our_str);
//                 serde_json::from_str::<Sprite>(&our_str).unwrap();
//             }
//         }
//     }

//     #[test]
//     fn deep_check() {
//         let sprite = include_str!("../../../data/sprites/test0.yy");
//         let sprite: Sprite =
//             serde_json::from_str(&TrailingCommaUtility::clear_trailing_comma_once(sprite)).unwrap();

//         let expected_sprite = Sprite {
//             bbox_mode: BBoxMode::Automatic,
//             collision_kind: CollisionKind::Rectangle,
//             resource_sprite_type: 0,
//             origin: Origin::Custom,
//             pre_multiply_alpha: false,
//             edge_filtering: false,
//             collision_tolerance: 0,
//             swf_precision: 2.525,
//             bbox_left: 7,
//             bbox_right: 38,
//             bbox_top: 5,
//             bbox_bottom: 47,
//             h_tile: false,
//             v_tile: false,
//             for3d: false,
//             width: NonZeroUsize::new(48).unwrap(),
//             height: NonZeroUsize::new(48).unwrap(),
//             texture_group_id: TexturePath {
//                 name: "Default".to_string(),
//                 path: TexturePathLocation("texturegroups/Default".to_string()),
//             },
//             swatch_colours: serde_json::Value::Null,
//             grid_x: 0,
//             grid_y: 0,
//             frames: vec![
//                 Frame {
//                     composite_image: Image {
//                         frame_id: FilesystemPath {
//                             name: "7669a695-40b5-47eb-a089-f81c0f6be6b8".to_string(),
//                             path: Path::new("sprites/spr_jack/spr_jack.yy").to_owned(),
//                         },
//                         layer_id: None,
//                         name: Some("composite".to_string()),
//                         ..Image::default()
//                     },
//                     images: vec![Image {
//                         frame_id: FilesystemPath {
//                             name: "7669a695-40b5-47eb-a089-f81c0f6be6b8".to_string(),
//                             path: Path::new("sprites/spr_jack/spr_jack.yy").to_owned(),
//                         },
//                         layer_id: Some(FilesystemPath {
//                             name: "37cbf63f-f6a9-4b91-a9fd-3537b374e9db".to_string(),
//                             path: Path::new("sprites/spr_jack/spr_jack.yy").to_owned(),
//                         }),
//                         name: None,
//                         ..Image::default()
//                     }],
//                     parent: FilesystemPath {
//                         name: "spr_jack".to_string(),
//                         path: Path::new("sprites/spr_jack/spr_jack.yy").to_owned(),
//                     },
//                     name: FrameId::with_id(
//                         uuid::Uuid::parse_str("7669a695-40b5-47eb-a089-f81c0f6be6b8").unwrap(),
//                     ),
//                     ..Default::default()
//                 },
//                 Frame {
//                     composite_image: Image {
//                         frame_id: FilesystemPath {
//                             name: "a52625ab-2499-4b46-9785-58ee50a1b048".to_string(),
//                             path: Path::new("sprites/spr_jack/spr_jack.yy").to_owned(),
//                         },
//                         layer_id: None,
//                         name: Some("composite".to_string()),
//                         ..Image::default()
//                     },
//                     images: vec![Image {
//                         frame_id: FilesystemPath {
//                             name: "a52625ab-2499-4b46-9785-58ee50a1b048".to_string(),
//                             path: Path::new("sprites/spr_jack/spr_jack.yy").to_owned(),
//                         },
//                         layer_id: Some(FilesystemPath {
//                             name: "37cbf63f-f6a9-4b91-a9fd-3537b374e9db".to_string(),
//                             path: Path::new("sprites/spr_jack/spr_jack.yy").to_owned(),
//                         }),
//                         name: None,
//                         ..Image::default()
//                     }],
//                     parent: FilesystemPath {
//                         name: "spr_jack".to_string(),
//                         path: Path::new("sprites/spr_jack/spr_jack.yy").to_owned(),
//                     },
//                     name: FrameId::with_id(
//                         uuid::Uuid::parse_str("a52625ab-2499-4b46-9785-58ee50a1b048").unwrap(),
//                     ),
//                     ..Default::default()
//                 },
//             ],
//             sequence: SpriteSequence {
//                 sprite_id: FilesystemPath {
//                     name: "spr_jack".to_string(),
//                     path: Path::new("sprites/spr_jack/spr_jack.yy").to_owned(),
//                 },
//                 playback_speed: 15.0,
//                 playback_speed_type: PlaybackSpeed::FramesPerSecond,
//                 length: 2.0,
//                 tracks: vec![Track {
//                     keyframes: {
//                         SpriteKeyframes {
//                             keyframes: vec![
//                                 SpriteKeyframe {
//                                     id: SpriteSequenceId::with_id(
//                                         uuid::Uuid::parse_str(
//                                             "7ac201d7-c56c-400f-8f97-c42ad0d98fba",
//                                         )
//                                         .unwrap(),
//                                     ),
//                                     key: 0.0,
//                                     channels: Channels {
//                                         zero: SpriteZeroChannel {
//                                             id: FilesystemPath {
//                                                 name: "7669a695-40b5-47eb-a089-f81c0f6be6b8"
//                                                     .to_string(),
//                                                 path: Path::new("sprites/spr_jack/spr_jack.yy")
//                                                     .to_owned(),
//                                             },
//                                             ..Default::default()
//                                         },
//                                     },
//                                     ..SpriteKeyframe::default()
//                                 },
//                                 SpriteKeyframe {
//                                     id: SpriteSequenceId::with_id(
//                                         uuid::Uuid::parse_str(
//                                             "d457b008-1cb9-43a5-8024-7576b09a0421",
//                                         )
//                                         .unwrap(),
//                                     ),
//                                     key: 1.0,
//                                     channels: Channels {
//                                         zero: SpriteZeroChannel {
//                                             id: FilesystemPath {
//                                                 name: "a52625ab-2499-4b46-9785-58ee50a1b048"
//                                                     .to_string(),
//                                                 path: Path::new("sprites/spr_jack/spr_jack.yy")
//                                                     .to_owned(),
//                                             },
//                                             ..SpriteZeroChannel::default()
//                                         },
//                                     },
//                                     ..SpriteKeyframe::default()
//                                 },
//                             ],
//                             ..SpriteKeyframes::default()
//                         }
//                     },
//                     ..Track::default()
//                 }],
//                 visible_range: Some(VisibleRange { x: 0.0, y: 0.0 }),
//                 lock_origin: false,
//                 backdrop_width: 1920,
//                 backdrop_height: 1080,
//                 backdrop_x_offset: 0.0,
//                 backdrop_y_offset: 0.0,
//                 xorigin: 23,
//                 yorigin: 42,
//                 parent: FilesystemPath {
//                     name: "spr_jack".to_string(),
//                     path: Path::new("sprites/spr_jack/spr_jack.yy").to_owned(),
//                 },
//                 ..SpriteSequence::default()
//             },
//             layers: vec![Layer {
//                 visible: true,
//                 is_locked: false,
//                 blend_mode: 0,
//                 opacity: 100.0,
//                 display_name: "Layer 1 (2)".to_string(),
//                 name: LayerId::with_string("37cbf63f-f6a9-4b91-a9fd-3537b374e9db"),
//                 ..Layer::default()
//             }],
//             parent: ViewPath {
//                 name: "Sprites".to_string(),
//                 path: ViewPathLocation("folders/Sprites.yy".to_string()),
//             },
//             name: "spr_jack".to_string(),
//             ..Sprite::default()
//         };

//         assert_eq!(sprite, expected_sprite);
//     }
// }
