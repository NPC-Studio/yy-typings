use super::yyp::ResourceType;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_default::SmartDefault;
use std::num::NonZeroUsize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sprite {
    /// Event GUID
    pub id: SpriteId,

    /// ModelName. Always GMSprite.
    pub model_name: ConstGmSprite,

    /// Version string. Currently 1.12.
    pub mvc: String,

    /// Resource Name
    pub name: String,

    // Collisions
    #[serde(rename = "bbox_bottom")]
    pub bbox_bottom: isize,
    #[serde(rename = "bbox_left")]
    pub bbox_left: isize,
    #[serde(rename = "bbox_right")]
    pub bbox_right: isize,
    #[serde(rename = "bbox_top")]
    pub bbox_top: isize,

    #[serde(rename = "bboxmode")]
    pub bbox_mode: BBoxMode,
    #[serde(rename = "colkind")]
    pub colkind: CollisionKind,
    pub sepmasks: bool,
    #[serde(rename = "coltolerance")]
    pub coltolerance: u8,
    pub edge_filtering: bool,

    #[serde(rename = "For3D")]
    pub for3d: bool,

    pub frames: Vec<Frame>,
    pub layers: Vec<Layer>,

    pub origin: Origin,
    pub origin_locked: bool,
    pub playback_speed: f64,
    pub playback_speed_type: PlaybackSpeed,
    pub premultiply_alpha: bool,

    pub texture_group_id: TextureGroupId,

    /// The type of sprite, whether a bitmap or a vector sprite.
    /// Right now, we don't handle this intelligently.
    #[serde(rename = "type")]
    pub resource_sprite_type: usize,
    pub swf_precision: f64,

    #[serde(rename = "HTile")]
    pub h_tile: bool,
    #[serde(rename = "VTile")]
    pub v_tile: bool,

    pub height: NonZeroUsize,
    pub width: NonZeroUsize,

    #[serde(rename = "xorig")]
    pub xorig: usize,
    #[serde(rename = "yorig")]
    pub yorig: usize,
    /// This is probably always null, unless you make a swatch,
    /// but why are you doing that! Just don't do that. Easy.
    pub swatch_colours: serde_json::Value,

    pub grid_x: usize,
    pub grid_y: usize,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            id: SpriteId::new(),
            model_name: Default::default(),
            mvc: "1.12".to_owned(),
            name: Default::default(),
            bbox_bottom: Default::default(),
            bbox_left: Default::default(),
            bbox_right: Default::default(),
            bbox_top: Default::default(),
            bbox_mode: Default::default(),
            colkind: Default::default(),
            sepmasks: Default::default(),
            coltolerance: Default::default(),
            edge_filtering: Default::default(),
            for3d: Default::default(),
            frames: Default::default(),
            layers: Default::default(),
            origin: Default::default(),
            origin_locked: Default::default(),
            playback_speed: Default::default(),
            playback_speed_type: Default::default(),
            premultiply_alpha: Default::default(),
            texture_group_id: Default::default(),
            resource_sprite_type: Default::default(),
            swf_precision: Default::default(),
            h_tile: Default::default(),
            v_tile: Default::default(),
            height: NonZeroUsize::new(1).unwrap(),
            width: NonZeroUsize::new(1).unwrap(),
            xorig: Default::default(),
            yorig: Default::default(),
            swatch_colours: Default::default(),
            grid_x: Default::default(),
            grid_y: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    /// The UUID of this Frame. Every SpriteImage within
    /// this Frame will have a FrameId which will be the same
    /// as this ID.
    pub id: FrameId,
    pub model_name: ConstGmSpriteFrame,

    /// Current always 1.0.
    pub mvc: String,

    /// This is the SpriteId for the ResourceSprite which owns this Frame.
    #[serde(rename = "SpriteId")]
    pub sprite_id: SpriteId,

    /// This is the actual image you'll see in the game.
    /// It's a composite of the images below. It's LayerID will
    /// always be UUID::default, or 0000...0000, but it's
    /// FrameID will always == Self.Id.
    pub composite_image: Image,

    /// These are the images which compose the composite image.
    /// At the minimum, there will be one Image. It's LayerID will
    /// correspond to the LayerId of a Sprite above.
    pub images: Vec<Image>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    /// This ID seems to referenced nowhere else, and may not have any independent
    /// usage. It does not reference anything else, at the minimum.
    pub id: ImageId,

    /// The model name is always "GMSpriteImage"
    pub model_name: ConstGmImage,

    /// Currently 1.0
    pub mvc: String,

    /// This always corresponds to the FrameId which this SpriteImage is within.
    #[serde(rename = "FrameId")]
    pub frame_id: FrameId,

    /// This always corresponds to the LayerId which this SpriteImage corresponds to.
    #[serde(rename = "LayerId")]
    pub layer_id: LayerId,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    /// This UUID corresponds to the SpriteImage LayerId UUID.
    pub id: LayerId,
    pub model_name: ConstGmImageLayer,

    /// Currently "1.0"
    pub mvc: String,

    /// This is the SpriteId for the ResourceSprite which owns this ImageLayer.
    #[serde(rename = "SpriteId")]
    pub sprite_id: SpriteId,

    pub blend_mode: usize,
    pub is_locked: bool,
    pub name: String,

    /// Between 0-100
    pub opacity: u8,
    pub visible: bool,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, SmartDefault)]
#[repr(u8)]
pub enum CollisionKind {
    Precise,
    #[default]
    Rectangle,
    Ellipse,
    Diamond,
    RotatedRectangle = 5,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, SmartDefault)]
#[repr(u8)]
pub enum PlaybackSpeed {
    #[default]
    FramesPerSecond,
    FramesPerGameFrame,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmSprite {
    #[serde(rename = "GMSprite")]
    #[default]
    GmSprite,
}

impl From<ConstGmSprite> for ResourceType {
    fn from(_: ConstGmSprite) -> Self {
        Self::GmSprite
    }
}

#[derive(Debug, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmImageLayer {
    #[serde(rename = "GMImageLayer")]
    #[default]
    GmImageLayer,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmSpriteFrame {
    #[serde(rename = "GMSpriteFrame")]
    #[default]
    GmSpriteFrame,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmImage {
    #[serde(rename = "GMSpriteImage")]
    #[default]
    GmSpriteImage,
}

create_guarded_uuid!(SpriteId);
create_guarded_uuid!(TextureGroupId);
create_guarded_uuid!(FrameId);
create_guarded_uuid!(ImageId);
create_guarded_uuid!(LayerId);

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, SmartDefault)]
#[repr(u8)]
pub enum BBoxMode {
    #[default]
    Automatic,
    FullImage,
    Manual,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, SmartDefault)]
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
