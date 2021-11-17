use super::{sprite_constants::*, FilesystemPath, ResourceVersion, Tags};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_default::SmartDefault;

create_guarded_uuid!(FrameId);
create_guarded_uuid!(LayerId);

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    /// This is the actual image you'll see in the game.
    /// It's a composite of the images below. It's LayerID will
    /// always be UUID::default, or 0000...0000, but it's
    /// FrameID will always == Self.Id.
    pub composite_image: Image,

    /// These are the images which compose the composite image.
    /// At the minimum, there will be one Image. It's LayerID will
    /// correspond to the LayerId of a Sprite above.
    pub images: Vec<Image>,

    /// This is the path to the sprite parent. It will always have the name
    /// of the Sprite name, and the path to the sprite `.yy` file.
    pub parent: FilesystemPath,

    /// The version of this particular resource.
    pub resource_version: ResourceVersion,
    /// This is the name of the Frame, which will always be a UUID.
    pub name: FrameId,
    /// These are the tags affixed to this frame, which is not possible.
    #[default(vec![])]
    pub tags: Tags,
    /// This is the type of Resource.
    pub resource_type: ConstGmSpriteFrame,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    /// Although named FrameId, this is actually the path to the the parent
    /// frame resource. The `name` field will correspond to the `Frame.name`
    /// field.
    #[serde(rename = "FrameId")]
    pub frame_id: FilesystemPath,

    /// This always corresponds to the LayerId which this SpriteImage
    /// corresponds to. It will be `None` in the case of the
    /// `composite_image` field -- otherwise, it will contain a valid path
    /// to the parent layer.
    #[serde(rename = "LayerId")]
    pub layer_id: Option<FilesystemPath>,

    /// The version of the resource.
    pub resource_version: ResourceVersion,

    /// This appears to only ever be two values:
    ///
    /// - `None` for normal images
    /// - `Some("composite")` for the composite image.
    ///
    /// It may have other purposes.
    pub name: Option<String>,

    /// The tags assigned to each image. You can apparently tag an image.
    pub tags: Tags,

    /// The resource name of the GM Image.
    pub resource_type: ConstGmImage,
}

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    /// Defines the visibility of the layer. It will default to true on
    /// import. It is changed in the GMS2 Sprite Editor.
    pub visible: bool,

    /// Defines if the layer is locked to editing. Only has an effect on the
    /// GMS2 Sprite Editor.
    pub is_locked: bool,

    /// Defines the blendmode in the GMS2 editor. @todo Must be typed at a later
    /// date.
    pub blend_mode: usize,

    /// Between 0-100
    #[default = 100.0]
    pub opacity: f64,

    /// This is the actual name shown in the GMS2 Sprite Editor.
    #[default("default".to_string())]
    pub display_name: String,

    /// Currently "1.0"
    pub resource_version: ResourceVersion,

    /// The legacy name of the LayerId.
    #[default(LayerId::new())]
    pub name: LayerId,

    /// The tags assigned to each layer.
    pub tags: Tags,

    /// The name of the Resource Type, which is always "GMImageLayer".
    pub resource_type: ConstGmImageLayer,
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
pub enum BlendMode {
    #[default]
    Normal,
    Add,
    Subtract,
    Multiply,
}
