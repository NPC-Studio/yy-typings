use super::yyp::ResourceType;
use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

create_guarded_uuid!(SpriteId);
create_guarded_uuid!(TextureGroupId);
create_guarded_uuid!(FrameId);
create_guarded_uuid!(SpriteImageId);
create_guarded_uuid!(LayerId);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sprite {
    /// Event GUID
    pub id: SpriteId,

    /// ModelName. Always GMSprite.
    pub model_name: ConstGmSprite,

    /// Version string. Currently 1.12.
    mvc: String,

    /// Resource Name
    name: String,

    // Collisions
    #[serde(rename = "bbox_bottom")]
    pub bbox_bottom: f64,
    #[serde(rename = "bbox_left")]
    pub bbox_left: f64,
    #[serde(rename = "bbox_right")]
    pub bbox_right: f64,
    #[serde(rename = "bbox_top")]
    pub bbox_top: f64,
    #[serde(rename = "bboxmode")]
    pub bboxmode: SpriteBBoxMode,
    #[serde(rename = "colkind")]
    pub colkind: SpriteCollisionKind,
    #[serde(rename = "coltolerance")]
    pub coltolerance: f64,
    pub edge_filtering: bool,

    #[serde(rename = "For3D")]
    pub for3d: bool,

    pub frames: Vec<Frame>,
    pub layers: Vec<ImageLayer>,

    pub grid_x: f64,
    pub grid_y: f64,
    pub height: f64,

    #[serde(rename = "HTile")]
    pub h_tile: bool,

    pub origin: f64,
    pub origin_locked: bool,
    pub playback_speed: f64,
    pub playback_speedtype: SpritePlaybackSpeed,
    pub premultiply_alpha: bool,

    pub sepmasks: bool,

    /// This is always null...I think.
    pub swatch_colours: Option<()>,

    pub swf_precision: f64,

    pub texture_group_id: TextureGroupId,

    /// This is always 0. Why is it there? Who can know.
    #[serde(rename = "type", default = "sprite_type")]
    pub resource_sprite_type: usize,

    #[serde(rename = "VTile")]
    pub v_tile: bool,

    pub width: f64,
    #[serde(rename = "xorig")]
    pub xorig: f64,
    #[serde(rename = "yorig")]
    pub yorig: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConstGmSprite {
    #[serde(rename = "GMSprite")]
    GmSprite,
}

impl From<ConstGmSprite> for ResourceType {
    fn from(_: ConstGmSprite) -> Self {
        Self::GmSprite
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
    pub composite_image: SpriteImage,

    /// These are the images which compose the composite image.
    /// At the minimum, there will be one Image. It's LayerID will
    /// correspond to the LayerId of a Sprite above.
    pub images: Vec<SpriteImage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConstGmSpriteFrame {
    #[serde(rename = "GMSpriteFrame")]
    GmSpriteFrame,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpriteImage {
    /// This ID seems to referenced nowhere else, and may not have any independent
    /// usage. It does not reference anything else, at the minimum.
    pub id: SpriteImageId,

    /// The model name is always "GMSpriteImage"
    pub model_name: ConstGmSpriteImage,

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
pub enum ConstGmSpriteImage {
    #[serde(rename = "GMSpriteImage")]
    GmSpriteImage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageLayer {
    /// This UUID corresponds to the SpriteImage LayerId UUID.
    pub id: SpriteImageId,
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
    pub opacity: f64,
    pub visible: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConstGmImageLayer {
    #[serde(rename = "GMImageLayer")]
    GmImageLayer,
}

#[derive(Debug, Serialize, Deserialize, FromPrimitive, ToPrimitive)]
pub enum SpriteBBoxMode {
    Automatic,
    FullImage,
    Manual,
}

#[derive(Debug, Serialize, Deserialize, FromPrimitive, ToPrimitive)]
pub enum SpriteCollisionKind {
    Precise,
    Rectangle,
    Ellipse,
    Diamond,
    RotatedRectangle = 5,
}

#[derive(Debug, Serialize, Deserialize, FromPrimitive, ToPrimitive)]
pub enum SpritePlaybackSpeed {
    FramesPerSecond,
    FramesPerGameFrame,
}

fn sprite_type() -> usize {
    0
}
