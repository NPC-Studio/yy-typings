pub use super::*;

mod sequence;
pub use sequence::*;

mod frames_layers;
pub use frames_layers::*;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_default::SmartDefault;
use std::num::NonZeroUsize;

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sprite {
    #[serde(flatten)]
    pub common_data: crate::CommonData<consts::Sprite>,

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
    pub frames: Vec<crate::CommonData<consts::SpriteFrame, FrameId, 1, 1>>,

    pub grid_x: usize,
    pub grid_y: usize,

    #[default(NonZeroUsize::new(1).unwrap())]
    pub height: NonZeroUsize,

    #[serde(rename = "HTile")]
    pub h_tile: bool,

    /// Each layer within each Frame for this Sprite File. Unless users use
    /// the built-in Sprite Editor, this will likely always have a .len() of
    /// 1, as there is always *one* layer. Layers are shared between frames.
    pub layers: Vec<SpriteLayer>,

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
    pub resource_type: consts::NineSliceData,

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

gm_const!(
    Sprite -> "GMSprite",
    ImageLayer -> "GMImageLayer",
    SpriteFrame -> "GMSpriteFrame",
    Image -> "GMSpriteBitmap",
    Sequence -> "GMSequence",
    SpriteEvent -> "KeyframeStore<MessageEventKeyframe>",
    SpriteMoment -> "KeyframeStore<MomentsEventKeyframe>",
    SpriteTrackName -> "frames",
    SpriteFramesTrack -> "GMSpriteFramesTrack",
    SpriteKeyframes -> "KeyframeStore<SpriteFrameKeyframe>",
    SpriteKeyframe -> "Keyframe<SpriteFrameKeyframe>",
    SpriteZeroChannel -> "SpriteFrameKeyframe",
    NineSliceData -> "GMNineSliceData"
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::TrailingCommaUtility;
    use include_dir::{include_dir, Dir, DirEntry};

    static ALL_SPRITES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/data/sprites");

    #[test]
    fn trivial_sprite_parsing() {
        let tcu = TrailingCommaUtility::new();

        for object_file in ALL_SPRITES.find("**/*.yy").unwrap() {
            if let DirEntry::File(file) = object_file {
                println!("parsing {}", file.path().display());
                let our_str = std::str::from_utf8(file.contents()).unwrap();
                let our_str = tcu.clear_trailing_comma(our_str);
                serde_json::from_str::<Sprite>(&our_str).unwrap();
            }
        }
    }
}
