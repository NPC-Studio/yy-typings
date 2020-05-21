use super::{sprite_constants::*, Frame, Layer, ParentPath, SpriteSequence, Tags};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_default::SmartDefault;
use std::num::NonZeroUsize;

#[derive(Debug, Serialize, Deserialize, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct Sprite {
    pub bbox_mode: BBoxMode,
    pub collision_kind: CollisionKind,
    pub separate_masks: bool,

    /// The type of sprite, whether a bitmap or a vector sprite.
    /// Right now, we don't handle this intelligently.
    #[serde(rename = "type")]
    pub resource_sprite_type: usize,

    pub origin: Origin,

    pub pre_multiply_alpha: bool,
    pub edge_filtering: bool,
    pub collision_tolerance: u8,
    pub swf_precision: f64,

    #[serde(rename = "bbox_left")]
    pub bbox_left: isize,
    #[serde(rename = "bbox_right")]
    pub bbox_right: isize,
    #[serde(rename = "bbox_top")]
    pub bbox_top: isize,
    #[serde(rename = "bbox_bottom")]
    pub bbox_bottom: isize,

    #[serde(rename = "HTile")]
    pub h_tile: bool,
    #[serde(rename = "VTile")]
    pub v_tile: bool,
    #[serde(rename = "For3D")]
    pub for3d: bool,

    #[default(NonZeroUsize::new(1).unwrap())]
    pub width: NonZeroUsize,
    #[default(NonZeroUsize::new(1).unwrap())]
    pub height: NonZeroUsize,

    /// This is the Path to the Texture Group Id.
    pub texture_group_id: ParentPath,

    /// This is probably always null, unless you make a swatch,
    /// but why are you doing that! Just don't do that. Easy.
    pub swatch_colours: serde_json::Value,
    pub grid_x: usize,
    pub grid_y: usize,

    /// Each frame within this Sprite File.
    pub frames: Vec<Frame>,

    /// The sequence assigned to each sprite.
    pub sequence: SpriteSequence,

    /// Each layer within each Frame for this Sprite File. Unless users use
    /// the built-in Sprite Editor, this will likely always have a .len() of
    /// 1, as there is always *one* layer. Layers are shared between frames.
    pub layers: Vec<Layer>,

    /// Defines the parent of the YY folder path.
    pub parent: ParentPath,

    /// Version string. Right now, this is loosely typed and ignored,
    /// but will be used in the future to aid in parsing.
    pub resource_version: String,

    /// The human readable name of the resource, such as `spr_player`.
    pub name: String,

    /// These are the tags assigned in the GMS2 Editor to each sprite.
    pub tags: Tags,
    /// ModelName. Always GMSprite.
    pub resource_type: ConstGmSprite,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, SmartDefault, Copy, Clone)]
#[repr(u8)]
pub enum CollisionKind {
    Precise,
    #[default]
    Rectangle,
    Ellipse,
    Diamond,
    RotatedRectangle = 5,
}

create_guarded_uuid!(SpriteId);

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, SmartDefault, Copy, Clone)]
#[repr(u8)]
pub enum BBoxMode {
    #[default]
    Automatic,
    FullImage,
    Manual,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, SmartDefault, Copy, Clone)]
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
